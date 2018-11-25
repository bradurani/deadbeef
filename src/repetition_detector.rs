use shakmaty::*;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash;
use utils::*;

#[derive(Clone, Debug, PartialEq)]
pub struct RepetitionDetector {
    map: HashMap<RepetitionPosition, u8, BuildHasherDefault<XxHash>>,
}

// contains the elements of the position that matter for threefold repetition according to the
// rules
//TODO do newer Shakmaty versions make this easier?
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct RepetitionPosition {
    board: Board,
    turn: Color,
    castles: Bitboard,
    ep_square: Option<Square>,
}

impl RepetitionPosition {
    pub fn new(position: &Chess) -> RepetitionPosition {
        RepetitionPosition {
            board: position.board().clone(),
            turn: position.turn(),
            castles: position.castling_rights(),
            ep_square: position.ep_square(),
        }
    }
}

impl RepetitionDetector {
    pub fn new(starting_position: &Chess) -> RepetitionDetector {
        let mut detector = RepetitionDetector {
            map: deterministic_hash_map(),
        };
        detector.record(starting_position);
        detector
    }

    pub fn clone_and_record(&self, position: &Chess) -> RepetitionDetector {
        let mut rd = self.clone();
        rd.record(position);
        rd
    }

    pub fn is_drawn(&self, position: &Chess) -> bool {
        *self.map.get(&RepetitionPosition::new(position)).unwrap() == 3
    }

    pub fn record(&mut self, position: &Chess) {
        let entry = self
            .map
            .entry(RepetitionPosition::new(position))
            .or_insert(0);
        *entry += 1;
        debug_assert!(*entry < 4);
    }
}

impl Default for RepetitionDetector {
    fn default() -> RepetitionDetector {
        RepetitionDetector::new(&Chess::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use game::Game;
    use setup::*;

    #[test]
    fn test_increments_count() {
        let position = &Chess::default();
        let mut rd = RepetitionDetector::default();

        rd.record(&position);
        assert_eq!(false, rd.is_drawn(position));
        // note the repetition detector registers starting position on start
        rd.record(&position);
        assert_eq!(true, rd.is_drawn(position));
    }

    #[test]
    fn test_all_actions_are_draw_by_threefold() {
        let position = parse_fen("q4rk1/5p2/8/6Q1/8/8/8/6K1 b - - 3 2");
        let mut repetition_detector = RepetitionDetector::new(&position);
        let drawing_position_1 = parse_fen("q4r1k/5p2/8/6Q1/8/8/8/6K1 w - - 4 3");
        let drawing_position_2 = parse_fen("q4r2/5p1k/8/6Q1/8/8/8/6K1 w - - 4 3");
        repetition_detector.record(&drawing_position_1);
        repetition_detector.record(&drawing_position_1);
        repetition_detector.record(&drawing_position_2);
        repetition_detector.record(&drawing_position_2);
        let allowed_actions = position.allowed_actions();
        for action in allowed_actions {
            let mut new_position = position.clone();
            new_position.play_safe(&action);
            repetition_detector.record(&new_position);
            assert!(true, repetition_detector.is_drawn(&new_position));
        }
    }
}
