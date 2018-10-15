use mcts::*;
use separator::Separatable;
use settings::*;
use shakmaty::*;
use stats::*;
use std::fmt;

const TREENODE_MAX_DISPLAY_DEPTH: u32 = 3;

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
        fn fmt_subtree(
            f: &mut fmt::Formatter,
            node: &TreeNode,
            indent_level: u32,
            max_indent_level: u32,
        ) -> fmt::Result {
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
            if indent_level < max_indent_level - 1 {
                for child in &node.children {
                    try!(fmt_subtree(f, child, indent_level + 1, max_indent_level));
                }
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

        fmt_subtree(f, self, 0, TREENODE_MAX_DISPLAY_DEPTH)
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn search_params(settings: &Settings) -> String {
            match settings.search_type {
                SearchType::Steps => format!("n_samples: {}", settings.n_samples),
                SearchType::Time => format!("time_per_move: {}", settings.time_per_move_ms),
                SearchType::Mate => "to mate".to_string(),
            }
        }

        writeln!(
            f,
            "SETTINGS: {} THREADS: {} C: {} SEED: {}",
            // self.starting_move_num,
            // self.starting_position.board(),
            search_params(self),
            self.ensemble_size,
            self.c,
            self.starting_seed,
        )
    }
}

impl fmt::Display for TreeStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "tree: nodes: {}, min_depth: {}, max_depth: {}, ns: {}\n\
            top: {}",
            self.nodes, self.min_depth, self.max_depth, self.ns, self.top_n)
        )
    }
}
