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
        writeln!(f, "{}", self.state)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "\n{}. \nTIME:  {}\nOTIME: {}\n",
            self.move_num(),
            match self.time_remaining {
                Some(ref t) => t.to_string(),
                None => "".to_string(),
            },
            match self.opponent_time_remaining {
                Some(duration) => format!("{:?}", duration),
                None => "".to_string(),
            }
        )?;
        self.position().board().clone().write_emoji(f)
    }
}

impl fmt::Display for RunStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n\
             MCTS:    nodes {}, iterations: {}, leafs: {}\n\
             TIME:    elapsed {:?}\n\
             PLAYOUT: evals: {}, {} e/s\n",
            self.nodes_created.separated_string(),
            self.iterations.separated_string(),
            self.leaf_nodes.separated_string(),
            self.elapsed(),
            self.evals_per_second(),
            self.evals
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
                NodeState::Empty => "EM ",
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
            parent_n: u32,
            indent_level: u8,
        ) -> fmt::Result {
            for _ in 0..indent_level {
                f.write_str("    ")?
            }
            match node.action.clone() {
                Some(a) => writeln!(
                    f,
                    "{}. {} {} q={} n={} m={} v={} w={}",
                    node.move_num(),
                    a.to_string().pad_to_width(7),
                    node.state,
                    node.q.to_string().pad_to_width(16),
                    node.n.to_string().pad_to_width(5),
                    // node.score().to_string().pad_to_width(8),
                    node.minimax.to_string().pad_to_width(6),
                    node.value.to_string().pad_to_width(6),
                    weight(node, parent_n, settings),
                )?,
                None => writeln!(
                    f,
                    "{}. Root {} q={} n={} m={} v={}",
                    node.move_num(),
                    node.state,
                    node.q.to_string().pad_to_width(16),
                    node.n.to_string().pad_to_width(5),
                    // node.score().to_string().pad_to_width(8),
                    node.minimax.to_string().pad_to_width(6),
                    node.value.to_string().pad_to_width(6),
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
            write!(f, "")
        }

        fmt_subtree(f, &self.node, self.settings, 0, 0)?;
        Ok(())
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
pub fn info_print_tree(node: &TreeNode, settings: &Settings) {
    info!("\n{}", DisplayTreeNode::new(node, settings));
}

pub fn debug_print_tree(node: &TreeNode, settings: &Settings) {
    if settings.print_tree {
        debug!("\n{}", DisplayTreeNode::new(node, settings));
    }
}
