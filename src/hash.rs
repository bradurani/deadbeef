use rand::rngs::SmallRng;
use rand::Rng;
use settings::*;
use shakmaty::*;
use std::fmt;
use std::sync::Once;
use utils::*;

pub const PAWN: u8 = 0;
pub const KNIGHT: u8 = 1 << 1;
pub const BISHOP: u8 = 2 << 1;
pub const ROOK: u8 = 3 << 1;
pub const QUEEN: u8 = 4 << 1;
pub const KING: u8 = 5 << 1;

pub const BK_CASTLE: u8 = 1;
pub const WK_CASTLE: u8 = BK_CASTLE << WHITE;
pub const BQ_CASTLE: u8 = 1 << 2;
pub const WQ_CASTLE: u8 = BQ_CASTLE << WHITE;

// bit masks for generating index
pub const PIECE: u8 = 0b1110;
pub const COLOR: u8 = 1;

pub const WHITE: u8 = COLOR;
pub const BLACK: u8 = 0;

static mut PIECE_KEYS: [u64; 64 * 6 * 2] = [0; 64 * 6 * 2];
static mut CASTLE_KEYS: [u64; 16] = [0; 16];
static mut EP_KEYS: [u64; 8] = [0; 8];
static mut COLOR_KEY: u64 = 0;

pub type CastlingRightsArray = [[Option<Square>; 2]; 2]; // used by shakmaty::Castles

static INIT: Once = Once::new();

fn set_random(arr: &mut [u64], rng: &mut SmallRng) {
    for elem in arr.iter_mut() {
        *elem = rng.gen();
    }
}

pub unsafe fn init_hash_keys(settings: Settings) {
    INIT.call_once(|| {
        let mut rng = seeded_rng(settings.starting_seed);
        set_random(&mut PIECE_KEYS, &mut rng);
        set_random(&mut CASTLE_KEYS, &mut rng);
        set_random(&mut EP_KEYS, &mut rng);
        COLOR_KEY = rng.gen();
    });
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Hash {
    pub val: u64,
}

pub type Squares = [u8; 64];

impl Hash {
    pub fn generate(position: &Chess) -> Self {
        let mut hash: Hash = Default::default();

        for (square, piece) in position.board().pieces() {
            hash.set_piece(square, piece);
        }
        hash.set_castling(position.castles());
        hash.set_ep(position.ep_square());
        if position.turn() == Color::White {
            hash.flip_color();
        }
        hash
    }

    pub fn set_piece(&mut self, sq: Square, piece: Piece) {
        let piece_rep = piece_rep(piece);
        let index = sq as usize
            + ((piece_rep & PIECE) >> 1) as usize * 64
            + (piece_rep & COLOR) as usize * 384;
        self.val ^= unsafe { PIECE_KEYS[index] };
    }

    pub fn set_castling(&mut self, castling: &Castles) {
        let mut castle_key: u8 = 0;
        if castling.has(Color::Black, CastlingSide::KingSide) {
            castle_key |= BK_CASTLE
        }
        if castling.has(Color::Black, CastlingSide::QueenSide) {
            castle_key |= BQ_CASTLE
        }
        if castling.has(Color::White, CastlingSide::KingSide) {
            castle_key |= WK_CASTLE
        }
        if castling.has(Color::White, CastlingSide::QueenSide) {
            castle_key |= WQ_CASTLE
        }
        unsafe {
            self.val ^= CASTLE_KEYS[castle_key as usize];
        }; // TODO will this work on 32 but systems?
    }

    pub fn set_ep(&mut self, en_passant: Option<Square>) {
        match en_passant {
            Some(square) => {
                let file = lsb(square as u64) % 8;
                self.val ^= unsafe { EP_KEYS[file as usize] };
            }
            _ => {}
        }
    }

    pub fn flip_color(&mut self) {
        self.val ^= unsafe { COLOR_KEY };
    }

    pub fn sub(&self) -> u16 {
        (self.val >> 48) as u16
    }
}

fn piece_rep(piece: Piece) -> u8 {
    // role_rep is always even, so the 1 digit is always 0 and we can | with color_rep
    // to get a unique value
    let role_rep = match piece.role {
        Role::Pawn => PAWN,
        Role::Knight => KNIGHT,
        Role::Bishop => BISHOP,
        Role::Rook => ROOK,
        Role::Queen => QUEEN,
        Role::King => KING,
    };
    let color_rep = match piece.color {
        Color::White => 1, // copied from crabby *shrug*
        Color::Black => 0,
    };
    role_rep | color_rep
}

pub fn lsb(val: u64) -> u32 {
    val.trailing_zeros()
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use setup::*;
    use shakmaty::fen::*;
    use std::collections::HashMap;
    use std::collections::HashSet;

    #[test]
    fn hashes_position() {
        unsafe { init_hash_keys(Settings::test_default()) };
        let position: Chess = Default::default();
        let hash = Hash::generate(&position);
        assert!(hash.val != 0);
    }

    #[test]
    fn differs_with_en_passant() {
        unsafe { init_hash_keys(Settings::test_default()) };
        let with_ep = parse_fen("r3k2r/pbppqpb1/1pn3p1/7p/1N2pPn1/1PP4N/PB1P2PP/2QRKR2 b kq f3");
        let without_ep = parse_fen("r3k2r/pbppqpb1/1pn3p1/7p/1N2pPn1/1PP4N/PB1P2PP/2QRKR2 b kq -");
        assert_ne!(
            Hash::generate(&with_ep).val,
            Hash::generate(&without_ep).val
        );
    }

    #[test]
    fn set_piece_is_reversible() {
        unsafe { init_hash_keys(Settings::test_default()) };
        let position = parse_fen("4k3/8/8/8/3P4/8/8/4K3 w - -");
        let hash = Hash::generate(&position);
        let mut new_hash = hash.clone();
        new_hash.set_piece(
            Square::D4,
            Piece {
                color: Color::White,
                role: Role::Pawn,
            },
        );
        assert_eq!(
            new_hash,
            Hash::generate(&parse_fen("4k3/8/8/8/8/8/8/4K3 w - -"))
        );
        new_hash.set_piece(
            Square::D4,
            Piece {
                color: Color::White,
                role: Role::Pawn,
            },
        );
        assert_eq!(hash, new_hash);
    }

    #[test]
    fn castling_rights_all_combinations() {
        unsafe { init_hash_keys(Settings::test_default()) };
        let mut seen: HashSet<u64> = HashSet::new();
        let variations = [
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w KQkq -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w KQk -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w KQq -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w Kkq -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w Qkq -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w KQ -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w kq -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w Kk -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w Qq -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w Kq -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w Qk -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w K -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w Q -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w k -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w q -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R w - -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b KQkq -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b KQk -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b KQq -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b Kkq -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b Qkq -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b KQ -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b kq -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b Kk -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b Qq -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b Kq -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b Qk -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b K -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b Q -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b k -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b q -",
            "r3k2r/1ppp1pp1/8/8/8/8/1PPP1PP1/R3K2R b - -",
        ];
        for v in &variations {
            let position = parse_fen(v);
            let hash = Hash::generate(&position);
            if seen.contains(&hash.val) {
                panic!("already seen {} for {:?}", hash, position);
            }
            seen.insert(hash.val);
        }
    }

    #[test]
    fn uniquely_hashes_for_every_2_king_and_1_rook_position() {
        unsafe { init_hash_keys(Settings::test_default()) };
        let mut seen: HashMap<u64, Chess> = HashMap::new();
        for color in &[Color::White, Color::Black] {
            for white_king in 0i8..63 {
                for black_king in 0i8..63 {
                    for white_rook in 0i8..63 {
                        let mut board = Board::empty();
                        board.set_piece_at(
                            unsafe { ::std::mem::transmute(white_king) },
                            Piece {
                                color: Color::White,
                                role: Role::King,
                            },
                            false,
                        );
                        board.set_piece_at(
                            unsafe { ::std::mem::transmute(black_king) },
                            Piece {
                                color: Color::Black,
                                role: Role::King,
                            },
                            false,
                        );
                        board.set_piece_at(
                            unsafe { ::std::mem::transmute(white_rook) },
                            Piece {
                                color: Color::White,
                                role: Role::Rook,
                            },
                            false,
                        );
                        let fen = Fen {
                            board: board,
                            turn: *color,
                            castling_rights: Bitboard::EMPTY,
                            ..Default::default()
                        };
                        match Position::from_setup(&fen) {
                            Ok(position) => {
                                let hash = Hash::generate(&position).val;
                                if seen.contains_key(&hash) {
                                    println!("{:?}", position.board());
                                    println!("{:?}", seen.get(&hash).unwrap().board());
                                    println!("---");
                                }
                                seen.insert(hash, position);
                            }
                            Err(_) => {}
                        };
                    }
                }
            }
        }
        assert_eq!(seen.len(), 379648);
    }
}
