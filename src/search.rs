use display::*;
use game::*;
use mcts::*;
use settings::*;
use shakmaty::*;
use stats::*;
use std::io::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use uct::*;
use utils::*;

type SafeTreeNode = Arc<Mutex<TreeNode>>;

pub fn search(
    root: TreeNode,
    game: &Chess,
    move_run_stats: &mut RunStats,
    settings: &Settings,
) -> TreeNode {
    println!("Starting {}", TreeStats::tree_stats(&root));

    let new_root = match &settings.search_type {
        SearchType::Steps => search_samples(root, &game, move_run_stats, settings),
        SearchType::Time => search_time(root, &game, move_run_stats, settings),
        SearchType::Mate => search_to_outcome(root, &game, move_run_stats, settings),
    };

    // println!("\nnew_root\n{}", new_root);
    println!("End: {}", TreeStats::tree_stats(&new_root));

    new_root
}

pub fn search_time(
    root: TreeNode,
    game: &Chess,
    move_run_stats: &mut RunStats,
    settings: &Settings,
) -> TreeNode {
    let mut samples_total = 0;
    let t0 = Instant::now();

    //TODO remove
    let mcts = &mut MCTS::new(settings);

    let mut n_samples = (mcts.iterations_per_ms * settings.time_per_move_ms)
        .max(settings.max_batch_size as f32)
        .min(settings.min_batch_size as f32) as usize;

    let mut new_root = root;
    while n_samples >= settings.min_batch_size as usize {
        let batch_t0 = Instant::now();

        let mut batch_run_stats: RunStats = Default::default();
        batch_run_stats.sample_batches = 1;

        new_root = search_threaded_batch(new_root, game, n_samples, &mut batch_run_stats, settings);
        samples_total += n_samples;

        let time_spent = t0.elapsed().as_millis() as f32;
        mcts.iterations_per_ms = (samples_total as f32) / time_spent;

        let time_left = settings.time_per_move_ms - time_spent;
        n_samples = (mcts.iterations_per_ms * time_left)
            .max(0.)
            .min(settings.max_batch_size as f32) as usize;

        let batch_time_spent = batch_t0.elapsed().as_millis();
        batch_run_stats.total_time = batch_time_spent as u64;
        // println!("Batch: {}", batch_run_stats);
        move_run_stats.add(&batch_run_stats);
        if new_root.is_decisive() {
            // println!("found decisive");
            break;
        }
    }

    println!("iterations_per_ms: {}", mcts.iterations_per_ms);
    new_root
}

pub fn search_to_outcome(
    root: TreeNode,
    game: &Chess,
    move_run_stats: &mut RunStats,
    settings: &Settings,
) -> TreeNode {
    assert!(settings.n_samples >= 5000);
    let new_root = search_samples(root, game, move_run_stats, settings);
    assert!(new_root.outcome.is_some());
    new_root
}

pub fn search_samples(
    root: TreeNode,
    game: &Chess,
    move_run_stats: &mut RunStats,
    settings: &Settings,
) -> TreeNode {
    assert!(settings.n_samples > 0);
    let batches = settings.n_samples as usize / settings.max_batch_size;
    let remainder = settings.n_samples as f32 % settings.max_batch_size as f32;
    println!("running {} batches and {} remainder", batches, remainder);
    let mut new_root = root;
    for i in 0..batches {
        print!(".");
        std::io::stdout().flush().unwrap();
        if i % 100 == 0 {
            // print!(".");
            print_tree(&new_root, settings);
        }
        new_root = search_threaded_batch(
            new_root,
            game,
            settings.max_batch_size,
            move_run_stats,
            settings,
        );
        move_run_stats.sample_batches += 1;
        if new_root.has_outcome() {
            return new_root;
        }
    }
    if remainder as usize >= settings.min_batch_size {
        new_root =
            search_threaded_batch(new_root, game, remainder as usize, move_run_stats, settings)
    }
    new_root
}

pub fn search_threaded_batch(
    mut root: TreeNode,
    game: &Chess,
    batch_n_samples: usize,
    batch_run_stats: &mut RunStats,
    settings: &Settings,
) -> TreeNode {
    debug_assert!(batch_n_samples >= settings.min_batch_size);
    debug_assert!(batch_n_samples <= settings.max_batch_size);
    debug_assert!(root.state != NodeState::LeafNode);

    // let coefficient = root.turn.coefficient();
    // let total_n = root.total_n();

    // sort_children_by_weight(&mut root.children, coefficient, total_n, settings);
    let mut new_root = root.clone_childless();
    ensure_expanded(&mut root, game, batch_run_stats, settings); //for a new game where root has no children, expand them
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

                if thread_num < thread_settings.threads {
                    // don't do work if we're over the thread count. Wastes spawing a thread :(
                    let t0 = Instant::now();

                    let mut thread_child = safe_thread_child.lock().unwrap();
                    thread_game.make_move(&thread_child.action.unwrap());

                    for _n in 0..batch_n_samples {
                        if thread_child.has_outcome() {
                            println!("found decisive in thread {}", thread_num);
                            break;
                        }
                        thread_run_stats.samples += 1;
                        new_q += thread_child.iteration(
                            &mut thread_game.clone(),
                            &mut rng,
                            &mut thread_run_stats,
                            &thread_settings,
                        );
                        new_n += 1.;
                    }
                    let time_spent = t0.elapsed().as_millis();
                    thread_run_stats.total_time = time_spent as u64;
                    // println!("thread: {}", thread_run_stats);
                    // println!("thread child: {:?}\n", thread_child);
                }
                (safe_thread_child.clone(), new_n, new_q, thread_run_stats) // only Arc reference is cloned
            })
        })
        .collect();

    let mut new_children: Vec<TreeNode> = thread_result_handles
        .into_iter()
        .map(|th| th.join().expect("panicked joining threads"))
        .map(|(safe_thread_child, new_n, new_q, thread_run_stats)| {
            // add stats from the children here, so we have a reference to new_root again
            batch_run_stats.add_thread_stats(&thread_run_stats, settings.threads);
            new_root.n += new_n;
            new_root.q += new_q;
            Arc::try_unwrap(safe_thread_child)
                .expect("unwraping arc")
                .into_inner()
                .expect("unwrapping mutex")
        })
        .collect();

    // set the outcome based on the children. Needs refactor
    for c in new_children.iter() {
        // this is inefficient because this method was designed to be used in iterate()
        new_root.set_outcome_based_on_child(c.outcome, c.min_score, c.max_score, batch_run_stats);
    }
    sort_children_by_weight(&mut new_children, new_root.n, settings);
    new_root.children = new_children;
    new_root
}

fn ensure_expanded(root: &mut TreeNode, game: &Chess, stats: &mut RunStats, settings: &Settings) {
    let mut rng = seeded_rng(settings.starting_seed);
    while ![NodeState::FullyExpanded, NodeState::LeafNode].contains(&root.state) {
        root.iteration(&mut game.clone(), &mut rng, stats, settings);
    }
}

#[cfg(test)]
mod tests {
    use search::*;
    use shakmaty::fen::Fen;
    use stats::{RunStats, TreeStats};

    #[test]
    #[ignore]
    fn search_deterministic_starting_pos() {
        fn run_search() -> TreeNode {
            let settings = Settings::lib_test_default();
            let mut test_run_stats: RunStats = Default::default();
            let game = &Chess::default();
            let root = TreeNode::new_root(game, 0.5);
            search(root, game, &mut test_run_stats, &settings)
        }
        let a = run_search();
        let b = run_search();
        let c = run_search();
        println!(
            "{:?}\n{:?}\n{:?}",
            TreeStats::tree_stats(&a),
            TreeStats::tree_stats(&b),
            TreeStats::tree_stats(&c)
        );
        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(a, c);
    }

    #[test]
    #[ignore]
    fn run_search_deterministic_middle_game_position() {
        fn run_search() -> TreeNode {
            let setup: Fen = "rn3rk1/pbppq1pp/1p2pb2/4N2Q/3PN3/3B4/PPP2PPP/R3K2R w KQ - 6 11"
                .parse()
                .unwrap();
            let game: Chess = setup.position().unwrap();
            let settings = Settings::lib_test_default();
            let root = TreeNode::new_root(&game, 1.);
            let mut test_run_stats: RunStats = Default::default();
            search(root, &game, &mut test_run_stats, &settings)
        }
        let a = run_search();
        let b = run_search();
        let c = run_search();
        println!(
            "{:?}\n{:?}\n{:?}",
            TreeStats::tree_stats(&a),
            TreeStats::tree_stats(&b),
            TreeStats::tree_stats(&c)
        );
        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(a, c);
    }
}
