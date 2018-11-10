use engine::*;
use log::*;
use mcts::*;
use pad::PadStr;
use separator::Separatable;
use settings::*;
use shakmaty::*;
use state::*;
use stats::*;
use std::fmt;
use uct::*;

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.state)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "\n{}. \nTIME:  {}\nOTIME: {}",
            self.ply_num, self.time_remaining, self.opponent_time_remaining
        )
    }
}

impl fmt::Display for RunStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n\
             PLAYOUTS:    moves {}, total: {} (avg: {})  maxouts: {}\n\
             NODES:       nodes {}, iterations: {}, leafs: {}\n\
             SAMPLES:     samples: {}, batches: {}\n\
             TIME:        playouts: {} ({:.*}%), other: {}({:.*}%), total: {}\n",
            self.playout_moves.separated_string(),
            self.playouts.separated_string(),
            if self.playouts == 0 {
                0
            } else {
                self.playout_moves / self.playouts
            },
            self.maxouts.separated_string(),
            self.nodes_created.separated_string(),
            self.iterations.separated_string(),
            self.leaf_nodes.separated_string(),
            self.samples.separated_string(),
            self.sample_batches.separated_string(),
            self.playout_time.separated_string(),
            1,
            self.playout_time_pct(),
            self.other_time().separated_string(),
            1,
            self.other_time_pct(),
            self.total_time
        )
    }
}

impl fmt::Display for NodeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NodeState::LeafNode => "LN ",
                NodeState::FullyExpanded => "FE ",
                NodeState::Expandable => "E  ",
            }
        )
    }
}

pub struct DisplayTreeNode<'a> {
    node: &'a TreeNode,
    settings: &'a Settings,
}

impl<'a> DisplayTreeNode<'a> {
    pub fn new(node: &'a TreeNode, settings: &'a Settings) -> DisplayTreeNode<'a> {
        DisplayTreeNode {
            node: node,
            settings: settings,
        }
    }
}

impl<'a> fmt::Display for DisplayTreeNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn fmt_subtree(
            f: &mut fmt::Formatter,
            node: &TreeNode,
            settings: &Settings,
            parent_n: f32,
            indent_level: u8,
        ) -> fmt::Result {
            for _ in 0..indent_level {
                f.write_str("    ")?;
            }
            match node.action {
                Some(a) => writeln!(
                    f,
                    "{}. {} {} q={} n={} s={} v={} w={} {} {} {}",
                    node.move_num,
                    a.to_string().pad_to_width(7),
                    node.state,
                    node.adjusted_q().to_string().pad_to_width(12),
                    node.n.to_string().pad_to_width(7),
                    node.color_relative_score().to_string().pad_to_width(8),
                    node.normalized_color_relative_value()
                        .to_string()
                        .pad_to_width(15),
                    weight(node, parent_n, settings),
                    format_max(node.max_score),
                    format_min(node.min_score),
                    format_outcome(node.outcome)
                )?,
                None => writeln!(
                    f,
                    "{}. Root {} q={} n={} s={} v={} {} {} {}",
                    node.move_num,
                    node.state,
                    node.adjusted_q().to_string().pad_to_width(12),
                    node.n.to_string().pad_to_width(7),
                    node.color_relative_score().to_string().pad_to_width(8),
                    node.normalized_color_relative_value()
                        .to_string()
                        .pad_to_width(15),
                    format_max(node.max_score),
                    format_min(node.min_score),
                    format_outcome(node.outcome)
                )?,
            }
            if indent_level < settings.max_tree_display_depth {
                for child in &node.children {
                    fmt_subtree(f, child, settings, node.n, indent_level + 1)?;
                }
            }
            fn format_max(max_score: Option<u16>) -> String {
                match max_score {
                    None => String::new(),
                    Some(m) => format!("max {}", m),
                }
            }
            fn format_min(min_score: Option<u16>) -> String {
                match min_score {
                    None => String::new(),
                    Some(m) => format!("min {}", m),
                }
            }
            fn format_outcome(outcome: Option<Outcome>) -> String {
                match outcome {
                    None => "".to_string(),
                    Some(o) => format!("OUTCOME={}", o),
                }
            }
            write!(f, "")
        }

        fmt_subtree(f, &self.node, self.settings, 0., 0)
    }
}

impl fmt::Display for TreeStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "tree: nodes: {}, min_depth: {}, max_depth: {}, ns: {}",
            self.nodes, self.min_depth, self.max_depth, self.ns
        )
    }
}

// TODO should be a macro
pub fn print_tree(node: &TreeNode, settings: &Settings) {
    if settings.print_tree {
        trace!("{}", DisplayTreeNode::new(node, settings));
    }
}
