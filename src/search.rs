use mcts::*;
use settings::*;
use shakmaty::*;
use stats::*;
use std::isize;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use tree_merge::timed_merge_trees;
use utils::*;

pub fn search(
    root: TreeNode,
    game: &Chess,
    move_run_stats: &mut RunStats,
    settings: &Settings,
) -> TreeNode {
    println!("Start {:?}", TreeStats::tree_stats(&root));

    let new_root = match &settings.search_type {
        SearchType::Steps => search_samples(root, &game, move_run_stats, settings),
        SearchType::Time => search_time(root, &game, move_run_stats, settings),
        SearchType::Mate => search_to_mate(root, &game, move_run_stats, settings),
    };

    // println!("new root {}", new_root);
    println!("End: {:?}", TreeStats::tree_stats(&new_root));
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
        n_samples = (mcts.iterations_per_ms * time_left).max(0.).min(100.) as usize;

        let batch_time_spent = batch_t0.elapsed().as_millis();
        batch_run_stats.total_time = batch_time_spent as u64;
        // println!("Batch: {}", batch_run_stats);
        move_run_stats.add(&batch_run_stats);
        if new_root.is_decisive() {
            println!("found decisive");
            break;
        }
    }

    println!("iterations_per_ms: {}", mcts.iterations_per_ms);

    new_root
}

pub fn search_to_mate(
    root: TreeNode,
    game: &Chess,
    move_run_stats: &mut RunStats,
    settings: &Settings,
) -> TreeNode {
    assert!(settings.n_samples == isize::MAX);
    search_samples(root, game, move_run_stats, settings)
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
    for _i in 0..batches {
        new_root = search_threaded_batch(
            new_root,
            game,
            settings.max_batch_size,
            move_run_stats,
            settings,
        );
        move_run_stats.sample_batches += 1;
        if new_root.is_decisive() {
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
    root: TreeNode,
    game: &Chess,
    batch_n_samples: usize,
    batch_run_stats: &mut RunStats,
    settings: &Settings,
) -> TreeNode {
    debug_assert!(batch_n_samples >= settings.min_batch_size);
    debug_assert!(batch_n_samples <= settings.max_batch_size);

    let thread_result_handles: Vec<JoinHandle<(TreeNode, RunStats)>> = (0..settings.ensemble_size)
        .map(|thread_num| {
            let thread_game = game.clone();
            let mut thread_root = root.clone();
            let mut rng = seeded_rng(settings.starting_seed + thread_num as u8);
            let mut thread_run_stats: RunStats = Default::default();
            let thread_settings = settings.clone();

            thread::spawn(move || {
                //Run iterations with playouts for this time slice
                let t0 = Instant::now();

                for _n in 0..batch_n_samples {
                    thread_run_stats.samples += 1;
                    thread_root.iteration(
                        &mut thread_game.clone(),
                        &mut rng,
                        &mut thread_run_stats,
                        &thread_settings,
                    );
                    if thread_root.is_decisive() {
                        println!("found decisive in thread {}", thread_num);
                        break;
                    }
                }
                let time_spent = t0.elapsed().as_millis();
                thread_run_stats.total_time = time_spent as u64;
                // println!("thread: {}", thread_run_stats);
                // println!("thread root: {}\n", thread_root);
                (thread_root, thread_run_stats)
            })
        })
        .collect();

    let (thread_roots, thread_run_stats) = thread_result_handles
        .into_iter()
        .map(|th| th.join().expect("panicked joining threads"))
        .fold(
            (vec![], vec![]),
            |(mut roots, mut stats), (thread_root, thread_run_stats)| {
                roots.push(thread_root);
                stats.push(thread_run_stats);
                (roots, stats)
            },
        );

    for stats in thread_run_stats {
        batch_run_stats.add_thread_stats(&stats, settings.ensemble_size);
    }

    timed_merge_trees(root, thread_roots.to_vec(), batch_run_stats)
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
            let settings = Settings::test_default();
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
            let settings = Settings::test_default();
            let mut mcts = MCTS::new(&settings);
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
