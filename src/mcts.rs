use eval::*;
use game::*;
use playout::*;
use rand::rngs::SmallRng;
use repetition_detector::RepetitionDetector;
use settings::*;
use shakmaty::*;
use stats::*;
use std::f32;
use std::i16;
use std::ops::Not;
use uct::*;
use utils::*;

// TODO do I need all these?
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum NodeState {
    LeafNode,
    FullyExpanded,
    Expandable,
    Empty, // placeholder so we can move something to threads for first move
}

impl Default for NodeState {
    fn default() -> NodeState {
        NodeState::Expandable
    }
}

// TODO do I need all these?
#[derive(Debug)]
pub struct TreeNode {
    pub action: Option<Move>, // how did we get here
    pub value: Reward,
    pub minimax: Reward,
    pub state: NodeState, // is this a leaf node? fully expanded?
    pub position: Chess,
    pub repetition_detector: RepetitionDetector,
    pub n: u32,                  //new qs computed during this search
    pub q: f32,                  //new qs computed during this search
    pub children: Vec<TreeNode>, // next steps we investigated
}

impl Default for TreeNode {
    fn default() -> TreeNode {
        TreeNode {
            action: None,
            value: 0,
            minimax: 0,
            state: NodeState::Expandable,
            position: Default::default(),
            repetition_detector: RepetitionDetector::default(),
            n: 0,
            q: 0.0,
            children: vec![],
        }
    }
}

//TODO, make all contructors take a game, and never allow manual setting of value
impl TreeNode {
    pub fn new_empty_child(action: Move, parent: &TreeNode) -> TreeNode {
        let mut position = parent.position.clone();
        position.make_move(&action);
        TreeNode {
            action: Some(action),
            position: position.clone(),
            repetition_detector: parent.repetition_detector.clone_and_record(&position),
            state: NodeState::Empty, // we're about to expand it in iteration()
            ..Default::default()
        }
    }

    pub fn new_root(position: Chess) -> TreeNode {
        TreeNode {
            position: position.clone(),
            repetition_detector: RepetitionDetector::new(&position),
            state: NodeState::Expandable, // don't want empty, because don't want to run a playout on it
            ..Default::default()
        }
    }

    pub fn clone_childless(&self) -> TreeNode {
        TreeNode {
            position: self.position.clone(),
            children: Vec::new(),
            repetition_detector: self.repetition_detector.clone(),
            n: self.n,
            q: self.q,
            minimax: self.minimax,
            value: self.value,
            state: self.state,
            action: self.action.clone(),
        }
    }

    pub fn color_relative_minimax(&self) -> i16 {
        self.minimax * self.turn().not().coefficient() as i16
    }

    pub fn color_relative_q(&self) -> f32 {
        self.q * self.turn().not().coefficient() as f32
    }

    pub fn color_relative_board_value(&self) -> Reward {
        // could save this calc, but don't think it's called much
        self.position.board().value() * self.turn().not().coefficient()
    }

    pub fn best_child_sort_value(&self) -> i32 {
        match self.state {
            // ensure we only choose this if all are Empty, then pick highest board value
            NodeState::Empty => -5000 + self.color_relative_board_value() as i32,
            NodeState::LeafNode => {
                if self.is_decisive() {
                    self.color_relative_minimax() as i32
                } else {
                    assert!(self.is_drawn());
                    self.n as i32
                }
            }
            _ => self.n as i32,
        }
    }

    pub fn is_decisive(&self) -> bool {
        self.state == NodeState::LeafNode && self.minimax != 0
    }

    pub fn move_num(&self) -> f32 {
        self.position.move_num()
    }

    pub fn turn(&self) -> Color {
        self.position.turn()
    }

    pub fn is_game_over(&self) -> bool {
        self.position.is_game_over()
            || self.position.halfmoves() == MAX_HALFMOVES
            || self.repetition_detector.is_drawn(&self.position)
    }

    pub fn is_drawn(&self) -> bool {
        self.position.halfmoves() == MAX_HALFMOVES
            || self.repetition_detector.is_drawn(&self.position)
            || self.position.is_stalemate()
            || self.position.is_insufficient_material()
    }

    pub fn has_winning_child(&self) -> bool {
        self.children.iter().any(|c| c.state == NodeState::LeafNode)
            && self.color_relative_minimax() < 0
    }

    pub fn all_children_leaf_nodes(&self) -> bool {
        self.children.iter().all(|c| c.state == NodeState::LeafNode)
    }

    pub fn outcome(&self) -> Option<Outcome> {
        if self.is_drawn() {
            return Some(Outcome::Draw);
        } else {
            return self.position.outcome();
        }
    }

    pub fn expand(&mut self, rng: &mut SmallRng, stats: &mut RunStats, settings: &Settings) -> f32 {
        // TODO, we can do better than random. Highest board value or value from transposition
        // table
        let candidate_actions = self.actions_with_no_children();
        // let action = choose_random(rng, &candidate_actions);
        let action = candidate_actions
            .iter()
            .max_by(|a, b| {
                let position_a = self.position.clone_and_play(a);
                let position_b = self.position.clone_and_play(b);
                position_a.board().value().cmp(&position_b.board().value())
            })
            .expect("no children to choose best from");

        let mut child = TreeNode::new_empty_child(action.clone(), self);
        if child.is_game_over() {
            child.value = child.outcome().expect("no outcome for child").reward();
            child.state = NodeState::LeafNode;
            stats.leaf_nodes += 1;
        } else {
            child.value = playout(self.position.clone(), stats, settings);
            child.state = NodeState::Expandable;
        }

        let normalized_value = child.normalized_value();
        child.q = normalized_value;
        child.minimax = child.value;
        child.n += 1;
        self.children.push(child);
        stats.nodes_created += 1;
        normalized_value
    }

    fn actions_with_no_children(&self) -> Vec<Move> {
        debug_assert!(self.position.legals().len() > 0);
        let child_actions: Vec<Move> = self
            .children
            .iter()
            .map(|c| c.action.clone().unwrap())
            .collect();
        self.position
            .legals()
            .into_iter()
            .filter(|a| !child_actions.contains(a))
            .collect()
    }

    pub fn iteration(
        &mut self,
        rng: &mut SmallRng,
        stats: &mut RunStats,
        settings: &Settings,
    ) -> f32 {
        stats.iterations += 1;
        debug_assert!(self.position.halfmoves() <= MAX_HALFMOVES);
        let normalized_value: f32 = match self.state {
            NodeState::FullyExpanded => {
                let normalized_value = {
                    let child = best_child(self, settings);
                    child.iteration(rng, stats, settings)
                };
                self.set_minimax_based_on_children();
                normalized_value
            }
            NodeState::Expandable => {
                let normalized_value = self.expand(rng, stats, &settings);
                normalized_value
            }
            NodeState::Empty => {
                self.value = playout(self.position.clone(), stats, settings);
                self.minimax = self.value;
                self.state = NodeState::Expandable;
                stats.nodes_created += 1;
                self.normalized_value()
            }
            NodeState::LeafNode => {
                panic!("IMPOSSIBLE LeafNode");
            }
        };
        self.check_fully_expanded();
        self.n += 1;
        self.q += normalized_value;
        normalized_value
    }

    pub fn check_fully_expanded(&mut self) {
        if [NodeState::Empty, NodeState::Expandable].contains(&self.state)
            && self.actions_with_no_children().is_empty()
        {
            self.state = NodeState::FullyExpanded;
            self.set_minimax_based_on_children();
        }
    }

    fn normalized_value(&self) -> f32 {
        (self.value as f32 / 9590.).min(1.) // (8 * 929) + (2 * 479) + (2 * 320) + (2 * 280)
                                            // TODO test 8 queen positions and other extremes
    }

    pub fn set_minimax_based_on_children(&mut self) {
        if self.state != NodeState::FullyExpanded && self.state != NodeState::LeafNode {
            return;
        }
        let new_minimax = self
            .children
            .iter()
            .map(|c| c.minimax)
            .max_by(|v1, v2| {
                let relative_v1 = v1 * self.turn().coefficient();
                let relative_v2 = v2 * self.turn().coefficient();
                relative_v1.cmp(&relative_v2)
            })
            .expect("no children to choose minimax from");
        self.minimax = new_minimax;
        if self.has_winning_child() || self.all_children_leaf_nodes() {
            self.state = NodeState::LeafNode;
            self.minimax -= self.turn().coefficient();
        }
    }

    pub fn generate_missing_children(&mut self) {
        // TODO, set the leaf node state in the TreeNode constructors
        for action in self.actions_with_no_children() {
            let node = TreeNode::new_empty_child(action, &self);
            self.children.push(node);
        }
    }
}
