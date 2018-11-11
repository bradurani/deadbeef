use game::*;
use mcts::*;
use settings::*;
use shakmaty::*;
use stats::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use uct::*;
use utils::*;

type SafeTreeNode = Arc<Mutex<TreeNode>>;

pub fn search_threaded(
    mut root: TreeNode,
    game: &Chess,
    stats: &mut RunStats,
    settings: &Settings,
) -> TreeNode {
    debug_assert!(root.state != NodeState::LeafNode);

    let n_threads = optimal_threads(root.children.len(), settings.max_threads);

    let mut new_root = root.clone_childless();
    ensure_expanded(&mut root, game, stats, &settings); //for a new game where root has no children, expand them
    new_root.state = NodeState::FullyExpanded;

    let thread_result_handles: Vec<JoinHandle<(SafeTreeNode, f32, f32, RunStats)>> = root
        .children
        .into_iter()
        // .take(settings.threads) // these are sorted by weight, so this favors most interesting nodes
        .map(|child| Arc::new(Mutex::new(child)))
        .enumerate()
        .map(|(thread_num, safe_thread_child)| {
            let mut thread_game = game.clone();
            let mut rng = seeded_rng(settings.starting_seed + thread_num as u8);
            let mut thread_run_stats: RunStats = Default::default();
            let thread_settings = settings.clone();

            thread::spawn(move || {
                let mut new_n = 0.;
                let mut new_q = 0.;

                if thread_num < n_threads {
                    // don't do work if we're over the thread count. Wastes spawing a thread :(
                    thread_run_stats.start_timer();

                    let mut thread_child = safe_thread_child.lock().unwrap();
                    thread_game.make_move(&thread_child.action.clone().unwrap());

                    for _n in 0..thread_settings.batch_size {
                        if thread_child.has_outcome() {
                            break;
                        }
                        thread_run_stats.nodes_created += 1;
                        new_q += thread_child.iteration(
                            &mut thread_game.clone(),
                            &mut rng,
                            &mut thread_run_stats,
                            &thread_settings,
                        );
                        new_n += 1.;
                    }
                    thread_run_stats.stop_timer();
                }
                (safe_thread_child.clone(), new_n, new_q, thread_run_stats) // only Arc reference is cloned
            })
        })
        .collect();

    let new_children: Vec<TreeNode> = thread_result_handles
        .into_iter()
        .map(|th| th.join().expect("panicked joining threads"))
        .map(|(safe_thread_child, new_n, new_q, thread_run_stats)| {
            // add stats from the children here, so we have a reference to new_root again
            stats.add(&thread_run_stats);
            new_root.n += new_n;
            new_root.q += new_q;
            Arc::try_unwrap(safe_thread_child)
                .expect("unwraping arc")
                .into_inner()
                .expect("unwrapping mutex")
        })
        .collect();

    new_root.children = new_children;
    new_root.set_outcome_from_children(stats);
    sort_children_by_weight(&mut new_root.children, new_root.n, settings);
    new_root
}

fn optimal_threads(n_children: usize, max_threads: u16) -> usize {
    (n_children / 2).min(max_threads as usize).max(1)
}

fn ensure_expanded(root: &mut TreeNode, game: &Chess, stats: &mut RunStats, settings: &Settings) {
    let mut rng = seeded_rng(settings.starting_seed);
    while ![NodeState::FullyExpanded, NodeState::LeafNode].contains(&root.state) {
        root.iteration(&mut game.clone(), &mut rng, stats, settings);
    }
}
