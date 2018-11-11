use settings::*;
use stats::*;

pub fn show_thinking(ply: u32, score: f32, stats: &RunStats, settings: &Settings) {
    let elapsed_cs = stats.elapsed().as_millis() / 10;
    if settings.show_thinking && stats.batches % 10 == 0 {
        println!("{} {} {} {}", ply, score, elapsed_cs, stats.nodes_created);
    }
}
