use game::*;
use shakmaty::*;
use std::f32;
use std::fmt;
use std::i32;
use std::ops::Not;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

use playout::playout;
use rand::rngs::SmallRng;
use settings::*;
use stats::*;
use tree_merge::timed_merge_trees;
use utils::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum NodeState {
    LeafNode,
    FullyExpanded,
    Expandable,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode {
    pub outcome: Option<Outcome>,
    pub action: Option<Move>, // how did we get here
    pub state: NodeState,     // is this a leaf node? fully expanded?
    //TODO don't need turn
    pub turn: Color, //which player made this move
    //TODO don't need move number
    pub move_num: f32,
    pub nn: f32,                 //new qs computed during this search
    pub nq: f32,                 //new qs computed during this search
    pub sn: f32,                 // saved n from previous searches
    pub sq: f32,                 // saved q from previous searchs. Used in UCT1, but not merged
    pub children: Vec<TreeNode>, // next steps we investigated
}

impl TreeNode {
    pub fn new(action: Option<Move>, turn: Color, move_num: f32) -> TreeNode {
        TreeNode {
            outcome: None,
            action: action,
            children: Vec::new(),
            state: NodeState::Expandable,
            turn: turn,
            move_num: move_num,
            nn: 0.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
        }
    }

    pub fn new_root(game: &Chess, move_num: f32) -> TreeNode {
        TreeNode {
            outcome: None,
            action: None,
            children: Vec::new(),
            state: NodeState::Expandable,
            turn: game.turn(),  // So we switch to White for move 1
            move_num: move_num, //So we increment to 1 for move 1
            nn: 0.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
        }
    }

    pub fn starting() -> TreeNode {
        TreeNode {
            outcome: None,
            action: None,
            children: Vec::new(),
            state: NodeState::Expandable,
            turn: Color::White,
            move_num: 0.5,
            nn: 0.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
        }
    }

    pub fn clone_empty(&self) -> TreeNode {
        TreeNode {
            outcome: self.outcome,
            action: self.action,
            children: Vec::new(),
            state: self.state,
            turn: self.turn,
            move_num: self.move_num,
            nn: 0.,
            nq: 0.,
            sn: 0.,
            sq: 0.,
        }
    }

    pub fn clone_childless(&self) -> TreeNode {
        TreeNode {
            outcome: self.outcome,
            action: self.action,
            children: Vec::new(),
            state: self.state,
            turn: self.turn,
            move_num: self.move_num,
            nn: 0.,
            nq: 0.,
            sn: self.sn,
            sq: self.sq,
        }
    }

    // saved ns from previous searches plus ns found in this search
    // used for UCT1 to guide search efforts
    pub fn total_n(&self) -> f32 {
        self.sn + self.nn
    }

    pub fn total_q(&self) -> f32 {
        self.sq + self.nq
    }

    pub fn score(&self) -> f32 {
        match self.outcome {
            Some(Outcome::Decisive { winner }) => {
                if winner == self.turn.not() {
                    f32::INFINITY
                } else {
                    f32::NEG_INFINITY
                }
            }
            Some(Outcome::Draw) => 0.,
            _ => self.sn,
        }
    }

    /// Find the best child accoring to UCT1
    pub fn best_child(&mut self, settings: &Settings) -> &mut TreeNode {
        let mut best_value: f32 = f32::NEG_INFINITY;
        let mut best_child: Option<&mut TreeNode> = None;
        let self_total_n = self.total_n();

        for child in &mut self.children {
            let value = (self.turn.coefficient() * child.total_q()) / child.total_n()
                + settings.c * (2. * self_total_n.ln() / child.total_n()).sqrt();
            //TODO what is this 2. ?????
            // println!("child: {:?} total: {}", child, child.total_n());
            // println!("value: {}", value);
            if value > best_value {
                best_value = value;
                best_child = Some(child);
            }
        }
        let found_best_child = best_child.unwrap();
        // println!("Best child for {:?}: {}", child_turn, found_best_child);
        found_best_child
    }

    /// Add a child to the current node with an previously unexplored action.
    pub fn expand(
        &mut self,
        candidate_actions: Vec<Move>,
        rng: &mut SmallRng,
        thread_run_stats: &mut RunStats,
    ) -> &mut TreeNode {
        // println!("Candidate Action: {:?}", &candidate_actions);

        let action = *choose_random(rng, &candidate_actions);

        self.children.push(TreeNode::new(
            Some(action),
            self.turn.not(),
            self.move_num + 0.5,
        ));
        thread_run_stats.nodes_created += 1;
        self.children.last_mut().unwrap()
    }

    fn candidate_actions(&self, allowed_actions: Vec<Move>) -> Vec<Move> {
        // What are our options given the current game state?
        // could save this between calls

        // Get a list with all the actions we tried alreday
        let mut child_actions: Vec<Move> = Vec::new();
        for child in &self.children {
            child_actions.push(child.action.expect("Child node without action"));
        }

        // Find untried actions
        let mut candidate_actions: Vec<Move> = Vec::new();
        for action in &allowed_actions {
            if !child_actions.contains(action) {
                candidate_actions.push(*action);
            }
        }
        candidate_actions
    }

    fn new_outcome_based_on_child(
        child_outcome: Option<Outcome>,
        turn: Color,
        children: &Vec<TreeNode>,
        game: &mut Chess,
    ) -> Option<Outcome> {
        match child_outcome {
            Some(Outcome::Decisive { winner: color }) if color == turn.not() => {
                println!("checking for child mate. Looking for grandchildren");
                println!("{:?}", game.board());
                if TreeNode::all_children_have_winning_grandchild(children, &game) {
                    Some(Outcome::Decisive { winner: turn.not() }) //can't escape checkmate. All move are a win for opponent
                } else {
                    None
                }
            }
            Some(Outcome::Decisive { winner: color }) if color == turn => {
                Some(Outcome::Decisive { winner: turn })
            }
            _ => None,
        }
    }

    /// Recursively perform an MCTS iteration.
    ///
    /// XXX A non-recursive implementation would probably be faster.
    /// XXX But how to keep &mut pointers to all our parents while
    /// XXX we fiddle with our leaf node?
    pub fn iteration(
        &mut self,
        game: &mut Chess,
        rng: &mut SmallRng,
        thread_run_stats: &mut RunStats,
        settings: &Settings,
    ) -> f32 {
        thread_run_stats.iterations += 1;
        // println!("{}", self);
        let mut delta = match self.state {
            NodeState::LeafNode => {
                // println!("{}", game.outcome().unwrap());
                // game.reward()
                self.outcome.unwrap().reward()
            }
            NodeState::FullyExpanded => {
                let (delta, outcome) = {
                    let child = self.best_child(settings);
                    let mut child_game = game.clone(); //TODO don't clone if the move is reversible
                    child_game.make_move(&child.action.unwrap());
                    let delta = child.iteration(&mut child_game, rng, thread_run_stats, settings);
                    println!("back from expand");
                    let outcome = child.outcome;
                    (delta, outcome)
                };

                let outcome_based_on_children = TreeNode::new_outcome_based_on_child(
                    outcome,
                    game.turn(),
                    &self.children,
                    game,
                );
                match outcome_based_on_children {
                    Some(Outcome::Decisive { winner: c }) => {
                        println!("set outcome based on children")
                    }
                    _ => {}
                }
                self.outcome = outcome_based_on_children;

                //we've now looked at the first grandchild node, which has propogated the win
                //back up if it's a checkmate for our opponent. Now check all of them to see if
                //we can't avoid mate in N
                delta
            }
            NodeState::Expandable => {
                let allowed_actions = game.allowed_actions();
                //TODO cleanup
                if allowed_actions.len() == 0 || game.is_insufficient_material() {
                    //TODO or 50 move rule
                    self.state = NodeState::LeafNode;
                    self.outcome = game.outcome();
                    return self.outcome.unwrap().reward();
                }
                let candidate_actions = self.candidate_actions(allowed_actions);
                println!("candidate actions {:?}", candidate_actions.len());
                if candidate_actions.len() == 0 {
                    //if we ended up expanded as the result fo a tree merge
                    //TODO check if merging filled out all outcomes
                    self.state = NodeState::FullyExpanded;
                    return self.iteration(game, rng, thread_run_stats, settings);
                }
                let mut child = self.expand(candidate_actions, rng, thread_run_stats);
                // println!("{:?}", child);
                game.make_move(&child.action.unwrap());
                if game.is_checkmate() {
                    println!("FOUND CHECKMATE");
                    println!("{:?}", game.board());
                    child.state = NodeState::LeafNode;
                    child.outcome = Some(Outcome::Decisive {
                        winner: game.turn().not(),
                    });
                    child.nn += 1.;
                    child.nq += 1.;
                    f32::INFINITY
                } else {
                    println!("playou");
                    let played_game = playout(rng, game, thread_run_stats);
                    let delta = played_game.outcome().map(|o| o.reward()).unwrap_or(0.);
                    child.nn += 1.;
                    child.nq += delta;
                    delta
                }
            }
        };
        if delta == f32::INFINITY {
            self.state = NodeState::LeafNode;
            delta = 1.;
            self.outcome = Some(Outcome::Decisive {
                winner: game.turn().not(), //we've advanced the game so turn is 1 ahead
            });
        }
        self.nn += 1.;
        self.nq += delta;
        delta
    }

    fn all_children_have_winning_grandchild(children: &Vec<TreeNode>, game: &Chess) -> bool {
        children.iter().all(|child| {
            match child.outcome {
                Some(Outcome::Decisive { winner: color }) if color == game.turn().not() => {
                    println!("found child mate");
                    println!("{:?}", child.action);
                    true
                }
                Some(_) => false, // stalemate or win
                None => {
                    let mut child_game = game.clone();
                    child_game.make_move(&child.action.unwrap());
                    println!("checking {:?}", child_game.board());
                    let allowed_actions = child_game.allowed_actions();
                    allowed_actions.iter().any(|aa| {
                        //TODO don't clone if the move is reversible
                        let mut grandchild_game = child_game.clone();
                        grandchild_game.make_move(aa);
                        println!("IS THIS A CHECKMATE? {:?}", grandchild_game.board());
                        if grandchild_game.is_checkmate() {
                            println!("{}", "found grandchild mate");
                        }
                        grandchild_game.is_checkmate()
                    })
                }
            }
        })
    }
}

impl fmt::Display for TreeNode {
    /// Output a nicely indented tree
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Nested definition for recursive formatting
        fn fmt_subtree(f: &mut fmt::Formatter, node: &TreeNode, indent_level: i32) -> fmt::Result {
            for _ in 0..indent_level {
                try!(f.write_str("    "));
            }
            match node.action {
                Some(a) => try!(writeln!(
                    f,
                    "{}. {} q={} n={} s={} {}",
                    node.move_num,
                    a,
                    node.total_q(),
                    node.total_n(),
                    node.score(),
                    format_outcome(node.outcome)
                )),
                None => try!(writeln!(
                    f,
                    "{}. Root q={} n={} s={} {}",
                    node.move_num,
                    node.total_q(),
                    node.total_n(),
                    node.score(),
                    format_outcome(node.outcome)
                )),
            }
            for child in &node.children {
                try!(fmt_subtree(f, child, indent_level + 1));
            }
            write!(f, "")
        }

        //TODO write to format buffer instead
        fn format_outcome(outcome: Option<Outcome>) -> String {
            match outcome {
                None => "".to_string(),
                Some(o) => format!("OUTCOME={}", o),
            }
        }

        fmt_subtree(f, self, 0)
    }
}

#[derive(Debug)]
pub struct MCTS {
    iterations_per_ms: f32,
    starting_seed: u8,
}

impl MCTS {
    pub fn new(settings: &Settings) -> MCTS {
        MCTS {
            iterations_per_ms: settings.starting_iterations_per_ms,
            starting_seed: settings.starting_seed,
        }
    }

    /// Perform n_samples MCTS iterations.
    pub fn search(
        &mut self,
        root: TreeNode,
        game: &Chess,
        batch_run_stats: &mut RunStats,
        settings: &Settings,
    ) -> TreeNode {
        // Iterate over ensemble and perform MCTS iterations
        let thread_result_handles: Vec<JoinHandle<(TreeNode, RunStats)>> = (0..settings
            .ensemble_size)
            .map(|thread_num| {
                let thread_game = game.clone();
                let mut thread_root = root.clone();
                let mut rng = seeded_rng(self.starting_seed + thread_num as u8);
                let mut thread_run_stats: RunStats = Default::default();
                let thread_settings = settings.clone();

                thread::spawn(move || {
                    //Run iterations with playouts for this time slice
                    let t0 = Instant::now();

                    for _ in 0..thread_settings.n_samples {
                        thread_run_stats.samples += 1;
                        thread_root.iteration(
                            &mut thread_game.clone(),
                            &mut rng,
                            &mut thread_run_stats,
                            &thread_settings,
                        );
                    }
                    let time_spent = t0.elapsed().as_millis();
                    thread_run_stats.total_time = time_spent as u64;
                    // println!("thread: {}", thread_run_stats);
                    println!("thread root: {}\n", thread_root);
                    (thread_root, thread_run_stats)
                })
            })
            .collect();

        let (thread_roots, thread_run_stats) = thread_result_handles
            .into_iter()
            .map(|th| th.join().expect("panicked joining threads"))
            .fold(
                (vec![], vec![]),
                |(mut roots, mut stats), (thread_root, thread_run_stats)| {
                    roots.push(thread_root);
                    stats.push(thread_run_stats);
                    (roots, stats)
                },
            );

        for stats in thread_run_stats {
            batch_run_stats.add_thread_stats(&stats, settings.ensemble_size);
        }

        timed_merge_trees(root, thread_roots.to_vec(), batch_run_stats)
    }

    pub fn search_time(
        &mut self,
        root: TreeNode,
        game: &Chess,
        move_run_stats: &mut RunStats,
        settings: &Settings,
    ) -> TreeNode {
        let mut samples_total = 0;
        let t0 = Instant::now();

        let mut n_samples = (self.iterations_per_ms * settings.time_per_move_ms)
            .max(10.)
            .min(100.) as usize;

        let mut new_root = root;
        while n_samples >= 5 {
            let batch_t0 = Instant::now();
            let mut batch_run_stats: RunStats = Default::default();
            batch_run_stats.sample_batches = 1;

            new_root = self.search(new_root, game, &mut batch_run_stats, settings);
            samples_total += n_samples;

            let time_spent = t0.elapsed().as_millis() as f32;
            self.iterations_per_ms = (samples_total as f32) / time_spent;

            let time_left = settings.time_per_move_ms - time_spent;
            n_samples = (self.iterations_per_ms * time_left).max(0.).min(100.) as usize;

            let batch_time_spent = batch_t0.elapsed().as_millis();
            batch_run_stats.total_time = batch_time_spent as u64;
            // println!("Batch: {}", batch_run_stats);
            move_run_stats.add(&batch_run_stats);
        }

        println!("iterations_per_ms: {}", self.iterations_per_ms);

        new_root
    }
}

#[cfg(test)]
mod tests {
    use mcts::*;
    use setup::*;
    use shakmaty::fen::Fen;
    use stats::TreeStats;

    #[test]
    #[ignore]
    fn search_deterministic_starting_pos() {
        fn run_search() -> TreeNode {
            let settings = Settings::test_default();
            let mut test_run_stats: RunStats = Default::default();
            let mut mcts = MCTS::new(&settings);
            let game = &Chess::default();
            let root = TreeNode::new_root(game, 0.5);
            mcts.search(root, game, &mut test_run_stats, &settings)
        }
        let a = run_search();
        let b = run_search();
        let c = run_search();
        println!(
            "{:?}\n{:?}\n{:?}",
            TreeStats::tree_stats(&a),
            TreeStats::tree_stats(&b),
            TreeStats::tree_stats(&c)
        );
        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(a, c);
    }

    #[test]
    #[ignore]
    fn run_search_deterministic_middle_game_position() {
        fn run_search() -> TreeNode {
            let setup: Fen = "rn3rk1/pbppq1pp/1p2pb2/4N2Q/3PN3/3B4/PPP2PPP/R3K2R w KQ - 6 11"
                .parse()
                .unwrap();
            let game: Chess = setup.position().unwrap();
            let settings = Settings::test_default();
            let mut mcts = MCTS::new(&settings);
            let root = TreeNode::new_root(&game, 1.);
            let mut test_run_stats: RunStats = Default::default();
            mcts.search(root, &game, &mut test_run_stats, &settings)
        }
        let a = run_search();
        let b = run_search();
        let c = run_search();
        println!(
            "{:?}\n{:?}\n{:?}",
            TreeStats::tree_stats(&a),
            TreeStats::tree_stats(&b),
            TreeStats::tree_stats(&c)
        );
        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(a, c);
    }

    #[test]
    fn iteration_mate_in_1() {
        let mut stats: RunStats = Default::default();
        let (node, score) =
            test_iteration_all_children_with_stats("4k3/Q7/5K2/8/8/8/8/8 w - - 0 1", &mut stats);
        assert_eq!(1., score);
        assert_eq!(
            Outcome::Decisive {
                winner: Color::White
            },
            node.outcome.unwrap()
        );
        println!("{:?}", stats);
    }

    #[test]
    fn iteration_mate_in_2() {
        let mut stats: RunStats = Default::default();
        let (node, score) =
            test_iteration_all_children_with_stats("4q3/8/8/8/8/3k4/8/3K4 b - - 0 1", &mut stats);
        assert_eq!(1., score);
        assert_eq!(
            Outcome::Decisive {
                winner: Color::Black
            },
            node.outcome.unwrap()
        );
        assert_eq!(stats.iterations, 178);
        println!("{:?}", stats);
    }

    #[test]
    fn iteration_mate_in_2_2_choices() {
        let mut stats: RunStats = Default::default();
        let (node, score) = test_iteration_all_children_with_stats(
            "8/5Q2/1pkq2n1/pB2p3/4P3/1P2K3/2P5/8 b - - 1 1",
            &mut stats,
        );
        assert_eq!(1., score);
        assert_eq!(
            Outcome::Decisive {
                winner: Color::White
            },
            node.outcome.unwrap()
        );
        assert_eq!(stats.iterations, 45);
        println!("{:?}", stats);
    }

    fn test_iteration_all_children_with_stats(
        fen_str: &'static str,
        stats: &mut RunStats,
    ) -> (TreeNode, f32) {
        let game = parse_fen(fen_str);
        let mut settings = Settings::test_default();
        let mut rng = seeded_rng(settings.starting_seed);
        let mut node = TreeNode::new_root(&game, 1.);
        let mut last = 0.;
        let mut counter = 0;
        while node.outcome.is_none() {
            last = node.iteration(&mut game.clone(), &mut rng, stats, &mut settings);
            counter += 1;
            if counter > 100000 {
                println!("{}", node);
                panic!("did not find checkmate");
            }
        }
        println!("found {:?}", node.outcome);
        (node, last)
    }

    fn test_iteration_all_children(fen_str: &'static str) -> (TreeNode, f32) {
        let mut stats: RunStats = Default::default();
        test_iteration_all_children_with_stats(fen_str, &mut stats)
    }
}
