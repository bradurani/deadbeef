use bloom::BloomFilter;
use shakmaty::Board;

#[derive(Debug, PartialEq, Clone)]
pub struct RepetitionDetector {
    once: BloomFilter,
    twice: BloomFilter,
}

impl RepetitionDetector {
    pub fn default() -> RepetitionDetector {
        RepetitionDetector {
            once: bloom_filter(),
            twice: bloom_filter(),
        }
    }

    pub fn record_and_check(&self, board: &Board) -> u8 {
        match self.once.contains(board) {
            false => {
                self.once.insert(board);
                1
            }
            true => match self.twice.contains(board) {
                false => {
                    self.twice.insert(board);
                    2
                }
                true => 3,
            },
        }
    }
}

fn bloom_filter() -> BloomFilter {
    let mut filter: BloomFilter = BloomFilter::with_rate(0.0001, 300);
    filter
}
