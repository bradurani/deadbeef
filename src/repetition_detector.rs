use shakmaty::Board;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash;
use utils::*;

#[derive(Clone, Debug, PartialEq)]
pub struct RepetitionDetector {
    map: HashMap<Board, u8, BuildHasherDefault<XxHash>>,
}

impl RepetitionDetector {
    pub fn new() -> RepetitionDetector {
        RepetitionDetector {
            map: deterministic_hash_map(),
        }
    }

    pub fn create_with_starting(starting_board: &Board) -> RepetitionDetector {
        let mut detector = RepetitionDetector::new();
        detector.record_and_check(starting_board);
        detector
    }

    pub fn starting() -> RepetitionDetector {
        RepetitionDetector::create_with_starting(&Board::new())
    }

    pub fn record_and_check(&mut self, board: &Board) -> bool {
        let entry = self.map.entry(board.clone()).or_insert(0);
        *entry += 1;
        debug_assert!(*entry < 4);
        *entry == 3
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_increments_count() {
        let mut rd = RepetitionDetector::starting();
        assert_eq!(false, rd.record_and_check(&Board::new()));
        assert_eq!(true, rd.record_and_check(&Board::new()));
    }
}