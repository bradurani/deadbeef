use emojify::DisplayEmojify;
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
        writeln!(f, "{}", self.state)?;
        if self.settings.print_tree {
            writeln!(
                f,
                "{}",
                DisplayTreeNode {
                    node: &self.state.root,
                    settings: &self.settings
                }
            )?
        }
        self.state.position.board().clone().write_emoji(f)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "\n{}. \nTIME:  {}\nOTIME: {}",
            self.ply_num(),
            match self.time_remaining {
                Some(ref t) => t.to_string(),
                None => "".to_string(),
            },
            match self.opponent_time_remaining {
                Some(duration) => format!("{:?}", duration),
                None => "".to_string(),
            }
        )
    }
}

impl fmt::Display for RunStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n\
             NODES:   nodes {}, iterations: {}, leafs: {}\n\
             TIME:    batches: {}  elapsed {:?}\n",
            self.nodes_created.separated_string(),
            self.iterations.separated_string(),
            self.leaf_nodes.separated_string(),
            self.batches.separated_string(),
            self.elapsed()
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
                f.write_str("    ")?
            }
            match node.action.clone() {
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
            match settings.max_tree_display_depth {
                Some(max_depth) if indent_level >= max_depth - 1 => {}
                _ => {
                    for child in node
                        .children
                        .iter()
                        .take(settings.max_tree_display_length.unwrap_or(u8::max_value()) as usize)
                    {
                        fmt_subtree(f, child, settings, node.n, indent_level + 1)?;
                    }
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
