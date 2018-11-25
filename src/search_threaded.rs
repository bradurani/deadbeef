use mcts::*;
use settings::*;
use stats::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use tree_node::*;
use uct::*;
use utils::*;

type SafeTreeNode = Arc<Mutex<TreeNode>>;

pub fn search_threaded(mut root: TreeNode, stats: &mut RunStats, settings: &Settings) -> TreeNode {
    assert!(root.is_searchable());

    let n_threads = optimal_threads(root.children.len(), settings.max_threads);

    let mut new_root = root.clone_childless();
    root.generate_missing_children(stats);
    sort_children_by_weight(&mut root.children, new_root.n, settings);

    let thread_result_handles: Vec<JoinHandle<(SafeTreeNode, Option<f32>, RunStats)>> = root
        .children
        .into_iter()
        .map(|child| Arc::new(Mutex::new(child)))
        .enumerate()
        .map(|(thread_num, safe_thread_child)| {
            let mut rng = seeded_rng(settings.starting_seed + thread_num as u8);
            let mut thread_stats: RunStats = Default::default();
            let thread_settings = settings.clone();

            thread::spawn(move || {
                let mut normalized_value = None;
                if thread_num < n_threads {
                    // don't do work if we're over the thread count. Wastes spawing a thread :(
                    thread_stats.start_timer();
                    let mut thread_child = safe_thread_child.lock().unwrap();
                    if thread_child.is_searchable() {
                        normalized_value = Some(thread_child.iteration(
                            &mut rng,
                            &mut thread_stats,
                            &thread_settings,
                        ));
                    }
                    thread_stats.stop_timer();
                }
                (safe_thread_child.clone(), normalized_value, thread_stats) // only Arc reference is cloned
            })
        })
        .collect();
    let new_children: Vec<TreeNode> = thread_result_handles
        .into_iter()
        .map(|th| th.join().expect("panicked joining threads"))
        .map(|(safe_thread_child, normalized_value, thread_stats)| {
            // add stats from the children here, so we have a reference to new_root again
            if normalized_value.is_some() {
                stats.add(&thread_stats);
                new_root.n += 1;
                new_root.q += normalized_value.unwrap();
            }
            Arc::try_unwrap(safe_thread_child)
                .expect("unwraping arc")
                .into_inner()
                .expect("unwrapping mutex")
        })
        .collect();
    new_root.children = new_children;
    new_root.update_root_based_on_children();
    new_root
}

fn optimal_threads(n_children: usize, max_threads: u16) -> usize {
    (n_children / 2).min(max_threads as usize).max(1)
}
