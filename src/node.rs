use game::*;
use hash::*;
use setup::*;
use shakmaty::*;

#[derive(Clone, Debug)]
pub struct Node {
    pub position: Chess,
    pub hash: Hash,
}

impl Node {
    pub fn new(position: Chess) -> Node {
        let hash = Hash::generate(&position);
        Node {
            position: position,
            hash: hash,
        }
    }

    pub fn from_fen(fen_str: &str) -> Result<Node, String> {
        Ok(Node::new(parse_fen_input(fen_str)?))
    }

    pub fn make_move(&mut self, action: &Move) {
        let turn = self.turn();
        self.update_hash(turn, action);
        self.maybe_rehash_castles(turn, action);
        self.position.make_move(action);
        self.maybe_rehash_castles(turn, action);
        self.hash.set_ep(self.position.ep_square()); // add ep if we just gained one
    }

    pub fn is_game_over(&self) -> bool {
        self.position.is_game_over()
    }

    pub fn turn(&self) -> Color {
        self.position.turn()
    }

    fn update_hash(&mut self, turn: Color, action: &Move) {
        let piece = Piece {
            color: turn,
            role: action.role(),
        };
        match action {
            Move::Normal {
                role,
                from,
                capture,
                to,
                promotion,
            } => {
                self.hash.set_piece(*from, piece); // remove from current square
                if let Some(captured) = capture {
                    let piece = Piece {
                        color: !turn,
                        role: *captured,
                    }; // TODO, does all this derefing have a performance cost?
                    self.hash.set_piece(*to, piece); // remove captured piece
                }
                if let Some(promoted) = promotion {
                    let piece = Piece {
                        color: turn,
                        role: *promoted,
                    };
                    self.hash.set_piece(*to, piece)
                } else {
                    self.hash.set_piece(*to, piece); // add at new square
                }
            }
            Move::EnPassant { from, to } => {
                self.hash.set_piece(*from, piece);
                self.hash.set_piece(*to, piece);
                let captured_piece = Piece {
                    color: !turn,
                    role: Role::Pawn,
                };
                let captured_square = Square::from_coords(to.file(), from.rank());
                self.hash.set_piece(captured_square, captured_piece);
            }
            Move::Castle { king, rook } => {
                let king_piece = Piece {
                    color: turn,
                    role: Role::King,
                };
                let rook_piece = Piece {
                    color: turn,
                    role: Role::Rook,
                };
                self.hash.set_piece(*king, king_piece);
                self.hash.set_piece(*rook, rook_piece);
                let castling_side = action.castling_side().unwrap();
                self.hash.set_piece(castling_side.king_to(turn), king_piece);
                self.hash.set_piece(castling_side.rook_to(turn), rook_piece);
            }
            _ => {}
        };
        self.hash.set_ep(self.position.ep_square()); // removes ep if we had it or noops
        self.hash.flip_color();
    }

    fn maybe_rehash_castles(&mut self, turn: Color, action: &Move) {
        let rehash = match action {
            Move::Normal { role, capture, .. } => {
                *role == Role::King || *role == Role::Rook || *capture == Some(Role::Rook)
            }
            Move::Castle { .. } => true,
            _ => false,
        };
        if rehash {
            self.hash.set_castling(self.position.castles());
        }
    }
}

impl Default for Node {
    fn default() -> Node {
        Node::new(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use random_move::*;
    use settings::*;
    use setup::*;
    use shakmaty::fen::*;
    use std::collections::HashMap;
    use utils::*;

    #[test]
    fn updates_hash_for_normal_move() {
        assert_hashes_match_for_move("4k3/8/8/8/3P4/8/8/4K3 w - -", "d4d5");
    }

    #[test]
    fn updates_hash_for_promotion() {
        // white
        assert_hashes_match_for_move("7k/3P4/2K5/8/8/8/8/8 w - -", "d7d8q");
        assert_hashes_match_for_move("7k/3P4/2K5/8/8/8/8/8 w - -", "d7d8r");
        assert_hashes_match_for_move("7k/3P4/2K5/8/8/8/8/8 w - -", "d7d8b");
        assert_hashes_match_for_move("7k/3P4/2K5/8/8/8/8/8 w - -", "d7d8n");
        // black
        assert_hashes_match_for_move("8/8/1k6/8/5K2/8/3p4/8 b - -", "d2d1q");
        assert_hashes_match_for_move("8/8/1k6/8/5K2/8/3p4/8 b - -", "d2d1r");
        assert_hashes_match_for_move("8/8/1k6/8/5K2/8/3p4/8 b - -", "d2d1b");
        assert_hashes_match_for_move("8/8/1k6/8/5K2/8/3p4/8 b - -", "d2d1n");
    }

    #[test]
    fn updates_hash_for_capture() {
        // white
        assert_hashes_match_for_move(
            "6k1/2n1b1r1/r1q1p1p1/2p1PpNp/1pP2P1P/p2RB3/PP1Q2P1/3R2K1 w - -",
            "g5e6",
        );
        assert_hashes_match_for_move(
            "6k1/2n1b1r1/r1q1p1p1/2p1PpNp/1pP2P1P/p2RB3/PP1Q2P1/3R2K1 w - -",
            "e3c5",
        );
        assert_hashes_match_for_move(
            "6k1/2n1b1r1/r1q1p1p1/2p1PpNp/1pP2P1P/p2RB3/PP1Q2P1/3R2K1 w - -",
            "d3a3",
        );
        assert_hashes_match_for_move(
            "6k1/2n1b1r1/r1q1p1p1/2p1PpNp/1pP2P1P/p2RB3/PP1Q2P1/3R2K1 w - -",
            "b2b3",
        );
        assert_hashes_match_for_move(
            "6k1/2n1b1r1/r1q1p1p1/2p1PpNp/1pP2P1P/p2RB3/PP1Q2P1/3R2K1 w - -",
            "d2b4",
        );
        // black
        assert_hashes_match_for_move(
            "6k1/2n1b1r1/r1q1p1p1/2p1PpNp/1pP2P1P/p2RB3/PP1Q2P1/3R2K1 b - -",
            "e7g5",
        );
        assert_hashes_match_for_move(
            "6k1/2n1b1r1/r1q1p1p1/2p1PpNp/1pP2P1P/p2RB3/PP1Q2P1/3R2K1 b - -",
            "c6g2",
        );
        assert_hashes_match_for_move(
            "6k1/2n1b1r1/r1q1p1p1/2p1PpNp/1pP2P1P/p2RB3/PP1Q2P1/3R2K1 b - -",
            "a3b2",
        );
    }

    #[test]
    fn updates_hash_for_en_passant() {
        // white
        assert_hashes_match_for_move("8/3p4/1k6/4Pp2/2p5/8/1P1P1K2/8 w - f6 0 2", "e5f6");
        assert_hashes_match_for_move("8/5p2/1k6/3pP3/2p5/8/1P1P1K2/8 w - d6 0 2", "e5d6");
        // black
        assert_hashes_match_for_move("8/3p1p2/1k6/4P3/1Pp5/8/3P1K2/8 b - b3 0 1", "c4b3");
        assert_hashes_match_for_move("8/3p1p2/1k6/4P3/2pP4/8/1P3K2/8 b - d3 0 1", "c4d3");
        // white don't capture it
        assert_hashes_match_for_move("8/3p4/1k6/4Pp2/2p5/8/1P1P1K2/8 w - f6 0 2", "e5e6");
        assert_hashes_match_for_move("8/5p2/1k6/3pP3/2p5/8/1P1P1K2/8 w - d6 0 2", "e5e6");
        // black don't capture it
        assert_hashes_match_for_move("8/3p1p2/1k6/4P3/1Pp5/8/3P1K2/8 b - b3 0 1", "c4c3");
        assert_hashes_match_for_move("8/3p1p2/1k6/4P3/2pP4/8/1P3K2/8 b - d3 0 1", "c4c3");
    }

    #[test]
    fn updates_hash_for_double_push() {
        // white
        assert_hashes_match_for_move("8/8/8/3k4/2p5/8/1P1P1K2/8 w - - 0 1", "b2b4");
        assert_hashes_match_for_move("8/8/8/3k4/2p5/8/1P1P1K2/8 w - - 0 1", "d2d4");
        // black
        assert_hashes_match_for_move("k7/2p1p3/8/3P4/8/8/6K1/8 b - - 0 1", "c7c5");
        assert_hashes_match_for_move("k7/2p1p3/8/3P4/8/8/6K1/8 b - - 0 1", "e7e5");
    }

    #[test]
    fn updates_hash_for_castles() {
        // white
        assert_hashes_match_for_move("r3k2r/8/8/8/8/8/8/R3K2R w KQkq -", "e1g1");
        assert_hashes_match_for_move("r3k2r/8/8/8/8/8/8/R3K2R w KQkq -", "e1c1");
        // black
        assert_hashes_match_for_move("r3k2r/8/8/8/8/8/8/R3K2R b KQkq -", "e8g8");
        assert_hashes_match_for_move("r3k2r/8/8/8/8/8/8/R3K2R b KQkq -", "e8c8");
    }

    #[test]
    fn incrementally_creates_hash() {
        unsafe { init_hash_keys(Settings::test_default()) };
        let mut rng = seeded_rng(Settings::test_default().starting_seed);
        for _i in 0..10000 {
            let mut node: Node = Default::default();
            while !node.is_game_over() {
                let prev_node = node.clone();
                node.make_random_move(&mut rng);
                if node.hash != Hash::generate(&node.position) {
                    panic!("hash mismatch for {} to {}", prev_node, node);
                }
            }
        }
    }

    fn assert_hashes_match_for_move(fen_str: &'static str, uci_str: &'static str) {
        unsafe { init_hash_keys(Settings::test_default()) };
        let mut node = Node::from_fen(fen_str).unwrap();
        let action = parse_uci(uci_str, &node.position);
        node.make_move(&action);
        assert_eq!(node.hash, Hash::generate(&node.position));
    }
}
