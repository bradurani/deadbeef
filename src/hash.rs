use rand::rngs::SmallRng;
use rand::Rng;
use settings::*;
use shakmaty::*;
use utils::*;

pub const PAWN: u8 = 0;
pub const KNIGHT: u8 = 1 << 1;
pub const BISHOP: u8 = 2 << 1;
pub const ROOK: u8 = 3 << 1;
pub const QUEEN: u8 = 4 << 1;
pub const KING: u8 = 5 << 1;

// bit masks for generating index
pub const PIECE: u8 = 0b1110;
pub const COLOR: u8 = 1;

static mut piece_keys: [u64; 64 * 6 * 2] = [0; 64 * 6 * 2];
static mut castle_keys: [u64; 16] = [0; 16];
static mut ep_keys: [u64; 8] = [0; 8];
static mut color_key: u64 = 0;

fn set_random(arr: &mut [u64], rng: &mut SmallRng) {
    for elem in arr.iter_mut() {
        *elem = rng.gen();
    }
}

pub unsafe fn init(settings: Settings) {
    let seed: &[usize] = &[0];
    let mut rng = seeded_rng(settings.starting_seed);
    set_random(&mut piece_keys, &mut rng);
    set_random(&mut castle_keys, &mut rng);
    set_random(&mut ep_keys, &mut rng);
    color_key = rng.gen();
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Hash {
    pub val: u64,
}

pub type Squares = [u8; 64];

impl Hash {
    pub fn generate(position: &Chess) -> Self {
        let mut hash: Hash = Default::default();

        for (square, piece) in position.board().pieces() {
            hash.set_piece(square as usize, piece_rep(piece));
        }
        hash.set_ep(position.ep_square());
        if position.turn() == Color::White {
            hash.flip_color();
        }
        hash
    }

    // pub fn init(sqs: &Squares, castling: u8, en_passant: u64, color: u8) -> Self {
    //     let mut hash = Hash { val: 0 };
    //
    //     for (i, &sq) in sqs.iter().enumerate() {
    //         hash.set_piece(i, sq);
    //     }
    //
    //     hash.set_castling(castling);
    //     hash.set_ep(en_passant);
    //     if color == WHITE {
    //         hash.flip_color()
    //     }
    //     hash
    // }

    pub fn set_piece(&mut self, pos: usize, sq: u8) {
        let index = pos + ((sq & PIECE) >> 1) as usize * 64 + (sq & COLOR) as usize * 384;
        self.val ^= unsafe { piece_keys[index] };
    }

    pub fn set_castling(&mut self, castling: Bitboard) {
        unsafe {
            let castle_key: usize = ::std::mem::transmute(castling);
            self.val ^= castle_keys[castle_key];
        }; // TODO with this work on 32 but systems?
    }

    pub fn set_ep(&mut self, en_passant: Option<Square>) {
        match en_passant {
            Some(square) => {
                let file = lsb(square as u64) % 8;
                self.val ^= unsafe { ep_keys[file as usize] };
            }
            _ => {}
        }
    }

    pub fn flip_color(&mut self) {
        self.val ^= unsafe { color_key };
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

#[cfg(test)]
mod tests {
    use super::*;
    use setup::*;
    use shakmaty::fen::*;
    use std::collections::HashMap;
    use std::panic::catch_unwind;

    #[test]
    fn hashes_position() {
        unsafe { init(Default::default()) };
        let position: Chess = Default::default();
        let hash = Hash::generate(&position);
        assert!(hash.val != 0);
    }

    #[test]
    fn differs_with_en_passant() {
        unsafe { init(Default::default()) };
        let with_ep = parse_fen("r3k2r/pbppqpb1/1pn3p1/7p/1N2pPn1/1PP4N/PB1P2PP/2QRKR2 b kq f3");
        let without_ep = parse_fen("r3k2r/pbppqpb1/1pn3p1/7p/1N2pPn1/1PP4N/PB1P2PP/2QRKR2 b kq -");
        assert_ne!(
            Hash::generate(&with_ep).val,
            Hash::generate(&without_ep).val
        );
    }

    #[test]
    fn castling_rights_kingside_white() {
        unsafe { init(Default::default()) };
        let with_rights =
            parse_fen("rnbqk1nr/1ppp1ppp/p3p3/6b1/5P2/P1N1P3/1PPP2PP/R1BQKBNR w KQkq -");
        let without_rights =
            parse_fen("rnbqk1nr/1ppp1ppp/p3p3/6b1/5P2/P1N1P3/1PPP2PP/R1BQKBNR w - -");
        assert_ne!(
            Hash::generate(&with_rights).val,
            Hash::generate(&without_rights).val
        );
    }

    #[test]
    fn uniquely_hashes_for_every_2_king_and_1_rook_position() {
        unsafe { init(Default::default()) };
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
                            Err(msg) => {}
                        };
                    }
                }
            }
        }
        assert_eq!(seen.len(), 379648);
    }
}
