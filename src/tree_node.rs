use eval::*;
use game::*;
use repetition_detector::RepetitionDetector;
use shakmaty::*;
use std::f32;
use std::i16;
use std::ops::Not;

// TODO do I need all these?
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NodeState {
    Empty, // placeholder so we can move something to threads for first move
    Expandable,
    FullyExpanded,
    FullySearched,
    LeafNode,
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

    pub fn best_child_sort_value(&self) -> f32 {
        match self.state {
            // ensure we only choose this if all are Empty, then pick highest board value
            NodeState::Empty => -5000. + self.color_relative_board_value() as f32,
            NodeState::LeafNode => {
                if self.is_decisive() {
                    self.color_relative_minimax() as f32
                } else {
                    assert!(self.is_drawn());
                    self.n as f32
                }
            }
            _ => self.n as f32 + self.q,
        }
    }

    pub fn is_checkmate(&self) -> bool {
        self.position.is_checkmate()
    }

    // checkmate discovered for either color
    pub fn is_decisive(&self) -> bool {
        self.color_relative_minimax() > MAX_REWARD - 100
    }

    pub fn searchable(&self) -> bool {
        ![NodeState::LeafNode, NodeState::FullySearched].contains(&self.state)
    }

    pub fn display_move_num(&self) -> String {
        self.position.display_move_num()
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

    pub fn outcome(&self) -> Option<Outcome> {
        if self.is_drawn() {
            return Some(Outcome::Draw);
        } else {
            return self.position.outcome();
        }
    }
}
