// use display::*;
use eval::Value;
use game::*;
// use playout::playout;
use rand::rngs::SmallRng;
use repetition_detector::RepetitionDetector;
use settings::*;
use shakmaty::*;
use stats::*;
use std::f32;
use std::ops::Not;
use uct::*;
use utils::*;

const MAX_VALUE: f32 = 900.;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum NodeState {
    LeafNode,
    FullyExpanded,
    Expandable,
}

impl Default for NodeState {
    fn default() -> NodeState {
        NodeState::Expandable
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeNode {
    pub outcome: Option<Outcome>,
    pub action: Option<Move>, // how did we get here
    pub value: i16,
    pub state: NodeState, // is this a leaf node? fully expanded?
    //TODO don't need turn
    pub turn: Color, //which player made this move
    //TODO don't need move number
    pub move_num: f32,
    pub repetition_detector: RepetitionDetector,
    pub n: f32,                  //new qs computed during this search
    pub q: f32,                  //new qs computed during this search
    pub children: Vec<TreeNode>, // next steps we investigated
    pub max_score: Option<u16>,
    pub min_score: Option<u16>,
}

//TODO, make all contructors take a game, and never allow manual setting of value
impl TreeNode {
    pub fn new(
        action: Option<Move>,
        turn: Color,
        move_num: f32,
        value: i16,
        rd: RepetitionDetector,
    ) -> TreeNode {
        TreeNode {
            action: action,
            turn: turn,
            move_num: move_num,
            value: value,
            repetition_detector: rd,
            ..Default::default()
        }
    }

    pub fn new_root(game: &Chess, move_num: f32) -> TreeNode {
        TreeNode {
            turn: game.turn(),  // So we switch to White for move 1
            move_num: move_num, //So we increment to 1 for move 1
            value: game.board().value(),
            repetition_detector: RepetitionDetector::new(game),
            ..Default::default()
        }
    }

    pub fn clone_childless(&self) -> TreeNode {
        TreeNode {
            outcome: self.outcome,
            action: self.action.clone(),
            children: Vec::new(),
            state: self.state,
            turn: self.turn,
            move_num: self.move_num,
            value: self.value,
            repetition_detector: self.repetition_detector.clone(),
            n: self.n,
            q: self.q,
            max_score: self.max_score,
            min_score: self.min_score,
        }
    }

    pub fn score(&self) -> f32 {
        match self.outcome {
            Some(Outcome::Decisive { winner }) => match winner {
                Color::White => f32::INFINITY,
                Color::Black => f32::NEG_INFINITY,
            },
            Some(Outcome::Draw) => 0.,
            _ => (self.turn.not().coefficient() * self.n),
        }
    }

    pub fn adjusted_q(&self) -> f32 {
        let mut adjusted_q = self.turn.not().coefficient() * self.q;
        adjusted_q = match self.min_score {
            Some(min) => adjusted_q.min(min as f32), // opponent can do no worse than min so we can do no better than min
            None => adjusted_q,
        };
        match self.max_score {
            Some(max) => adjusted_q.max(max as f32),
            None => adjusted_q,
        }
    }

    pub fn color_relative_score(&self) -> f32 {
        self.score() * self.turn.not().coefficient()
    }

    pub fn color_relative_value(&self) -> f32 {
        self.value as f32 * self.turn.not().coefficient()
    }

    pub fn normalized_value(&self) -> f32 {
        (self.value as f32 / MAX_VALUE)
    }

    pub fn normalized_color_relative_value(&self) -> f32 {
        (self.value as f32 / MAX_VALUE) * self.turn.not().coefficient()
    }

    pub fn is_decisive(&self) -> bool {
        self.outcome.map(|o| o.is_decisive()).unwrap_or(false)
    }

    pub fn has_outcome(&self) -> bool {
        self.outcome.is_some()
    }

    pub fn is_draw(&self) -> bool {
        match self.outcome {
            Some(Outcome::Draw) => true,
            _ => false,
        }
    }

    pub fn is_game_over_or_drawn(&self, game: &Chess) -> bool {
        game.is_game_over() || game.halfmoves() >= MAX_HALFMOVES
    }

    pub fn winner(&self) -> Option<Color> {
        self.outcome.and_then(|o| o.winner())
    }

    /// Add a child to the current node with an previously unexplored action.
    pub fn expand(
        &mut self,
        game: &mut Chess,
        candidate_actions: Vec<Move>,
        rng: &mut SmallRng,
        thread_run_stats: &mut RunStats,
    ) -> &mut TreeNode {
        let action = choose_random(rng, &candidate_actions);
        game.make_move(action);
        let new_rep = self.repetition_detector.clone();
        let new_node = TreeNode::new(
            Some(action.clone()),
            self.turn.not(),
            self.move_num + 0.5,
            game.board().value(),
            new_rep,
        );
        self.children.push(new_node);
        thread_run_stats.nodes_created += 1;
        self.children.last_mut().unwrap()
    }

    fn candidate_actions(&self, allowed_actions: Vec<Move>) -> Vec<Move> {
        // What are our options given the current game state?
        // could save this between calls

        // Get a list with all the actions we tried alreday
        let mut child_actions: Vec<Move> = Vec::new();
        for child in &self.children {
            child_actions.push(child.action.clone().expect("Child node without action"));
        }

        // Find untried actions
        let mut candidate_actions: Vec<Move> = Vec::new();
        for action in &allowed_actions {
            if !child_actions.contains(action) {
                candidate_actions.push(action.clone());
            }
        }
        candidate_actions
    }

    pub fn set_outcome_from_children(&mut self, stats: &mut RunStats) {
        if self
            .children
            .iter()
            .any(|c| c.outcome == Some(Outcome::Decisive { winner: self.turn }))
        {
            // one of the children is a winning move for this parent, so this node is a winner
            self.outcome = Some(Outcome::Decisive { winner: self.turn });
        } else if self.children.iter().all(|c| {
            c.outcome == Some(Outcome::Decisive {
                winner: self.turn.not(),
            })
        }) {
            // if all children can force a win for opponent, this node is win for opponent
            self.outcome = Some(Outcome::Decisive {
                winner: self.turn.not(),
            });
        } else if self.children.iter().all(|c| {
            c.outcome == Some(Outcome::Draw) || c.outcome == Some(Outcome::Decisive {
                winner: self.turn.not(),
            })
        }) {
            self.outcome = Some(Outcome::Draw);
            self.max_score = Some(0);
            self.min_score = Some(0);
        } else if self.children.iter().any(|c| c.max_score == Some(0)) {
            // one of my children allows me to force a draw, the move leading to this position is at
            // best, a draw for my opponent
            self.min_score = Some(0)
        }
        // no else because I don't belive this is mutually exclusive to the above condition
        if self.children.iter().all(|c| c.min_score == Some(0)) {
            // all of my children allow opponent to force a draw, so the move leading to this is
            // at worst a draw for my opponent
            self.max_score = Some(0)
        }
        if self.outcome.is_some() {
            self.state = NodeState::LeafNode;
            stats.leaf_nodes += 1;
        }
    }

    pub fn iteration(
        &mut self,
        game: &mut Chess,
        rng: &mut SmallRng,
        thread_run_stats: &mut RunStats,
        settings: &Settings,
    ) -> f32 {
        thread_run_stats.iterations += 1;
        debug_assert!(game.halfmoves() <= MAX_HALFMOVES);
        let delta = match self.state {
            NodeState::FullyExpanded => {
                let (delta, child_changes_outcome) = {
                    let child = best_child(self, settings);
                    let mut child_game = game.clone(); //TODO don't clone if the move is reversible
                    child_game.make_move(&child.action.clone().unwrap());
                    let delta = child.iteration(&mut child_game, rng, thread_run_stats, settings);
                    (
                        delta,
                        child.outcome.is_some()
                            || child.min_score.is_some()
                            || child.max_score.is_some(),
                    )
                };
                if child_changes_outcome {
                    // this calc gets repeated a lot unnecesarily and can be made more efficient
                    self.set_outcome_from_children(thread_run_stats);
                }
                delta
            }
            NodeState::Expandable => {
                let allowed_actions = game.allowed_actions();
                let candidate_actions = self.candidate_actions(allowed_actions);
                debug_assert!(candidate_actions.len() > 0);
                let mut last_child_expansion = false;
                let mut new_state = self.state;

                if candidate_actions.len() == 1 {
                    new_state = NodeState::FullyExpanded;
                    last_child_expansion = true;
                }

                //advances game to position after action
                let (delta, outcome, min_score, node_state) = {
                    let mut child = self.expand(game, candidate_actions, rng, thread_run_stats);
                    if game.halfmoves() == MAX_HALFMOVES
                        || child.repetition_detector.record_and_check(game)
                        || game.is_stalemate()
                        || game.is_insufficient_material()
                    {
                        child.state = NodeState::LeafNode;
                        thread_run_stats.leaf_nodes += 1;
                        child.outcome = Some(Outcome::Draw);
                        child.n += 1.;
                        (0., None, Some(0), new_state)
                    } else if game.is_checkmate() {
                        child.state = NodeState::LeafNode;
                        child.outcome = game.outcome();
                        thread_run_stats.leaf_nodes += 1;
                        let delta = child.outcome.unwrap().reward();
                        child.n += 1.;
                        child.q += delta;
                        (delta, child.outcome, None, NodeState::LeafNode)
                    } else {
                        // let played_game = playout(rng, game, thread_run_stats);
                        // let delta = played_game.outcome().map_or(0., |o| o.reward());
                        child.n += 1.;
                        let delta = child.normalized_value(); //delta + child.normalized_value();
                        child.q += delta;
                        (delta, None, None, new_state)
                    }
                };
                if self.min_score == None {
                    self.min_score = min_score;
                }
                if self.outcome == None {
                    self.outcome = outcome;
                }
                self.state = node_state;
                if last_child_expansion {
                    self.set_outcome_from_children(thread_run_stats)
                }
                delta
            }
            _ => {
                panic!("IMPOSSIBLE ITERATION");
            }
        };
        self.n += 1.;
        self.q += delta;
        delta
    }
}
