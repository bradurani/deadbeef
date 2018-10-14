use mcts::*;
use separator::Separatable;
use shakmaty::*;
use stats::*;
use std::fmt;

impl fmt::Display for RunStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n\
             PLAYOUTS:    moves {}, total: {} (avg: {})  maxouts: {}\n\
             NODES:       nodes {}, iterations: {}, leafs: {}\n\
             SAMPLES:     samples: {}, batches: {}\n\
             TREE_MERGES: {}\n\
             TIME:        playouts: {} ({:.*}%), tree_merge: {} ({:.*}%), other: {}({:.*}%), total: {}\n",
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
            self.tree_merges.separated_string(),
            self.playout_time.separated_string(),
            1,
            self.playout_time_pct(),
            self.tree_merge_time.separated_string(),
            1,
            self.tree_merge_time_pct(),
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
                NodeState::LeafNode => "LN",
                NodeState::FullyExpanded => "FE",
                NodeState::Expandable => "E",
            }
        )
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
                    "{}. {} {} q={} n={} s={} {}",
                    node.move_num,
                    a,
                    node.state,
                    node.total_q(),
                    node.total_n(),
                    node.score(),
                    format_outcome(node.outcome)
                )),
                None => try!(writeln!(
                    f,
                    "{}. Root {} q={} n={} s={} {}",
                    node.move_num,
                    node.state,
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
