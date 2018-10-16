use core::iter::Iterator;
use game::*;
use shakmaty::*;

#[rustfmt::skip]
const PAWN_VALUES: [i16; 64] = [
    0,   0,   0,   0,   0,   0,   0,   0,
    78,  83,  86,  73, 102,  82,  85,  90,
    7,   29,  21,  44,  40,  31,  44,   7,
    -17, 16,  -2,  15,  14,   0,  15, -13,
    -26, 3,  10,   9,   6,   1,   0, -23,
    -22, 9,   5,  -11, -10,  -2,   3, -19,
    -31, 8,  -7,  -37, -36, -14,   3, -31,
    0,   0,  0,   0,   0,   0,     0, 0
];

#[rustfmt::skip]
const KNIGHT_VALUES: [i16; 64] = [
    -66, -53, -75, -75, -10, -55, -58, -70,
    -3,  -6, 100, -36,   4,  62,  -4,  -14,
    10,  67,   1,  74,  73,  27,  62,  -2,
    24,  24,  45,  37,  33,  41,  25,  17,
    -1,   5,  31,  21,  22,  35,   2,   0,
    -18,  10,  13,  22,  18,  15,  11, -14,
    -23, -15,   2,   0,   2,   0, -23, -20,
    -74, -23, -26, -24, -19, -35, -22, -69
];

#[rustfmt::skip]
const BISHOP_VALUES: [i16; 64] = [
    -59, -78, -82, -76, -23,-107, -37, -50,
    -11,  20,  35, -42, -39,  31,   2, -22,
    -9,  39, -32,  41,  52, -10,  28, -14,
    25,  17,  20,  34,  26,  25,  15,  10,
    13,  10,  17,  23,  17,  16,   0,   7,
    14,  25,  24,  15,   8,  25,  20,  15,
    19,  20,  11,   6,   7,   6,  20,  16,
    -7,   2, -15, -12, -14, -15, -10, -10,
];

#[rustfmt::skip]
const ROOK_VALUES: [i16; 64] = [
    35,  29,  33,   4,  37,  33,  56,  50,
    55,  29,  56,  67,  55,  62,  34,  60,
    19,  35,  28,  33,  45,  27,  25,  15,
    0,   5,  16,  13,  18,  -4,  -9,  -6,
    -28, -35, -16, -21, -13, -29, -46, -30,
    -42, -28, -42, -25, -25, -35, -26, -46,
    -53, -38, -31, -26, -29, -43, -44, -53,
    -30, -24, -18,   5,  -2, -18, -31, -32,
];

#[rustfmt::skip]
const QUEEN_VALUES: [i16; 64] =[
    6,   1,  -8,-104,  69,  24,  88,  26,
    14,  32,  60, -10,  20,  76,  57,  24,
    -2,  43,  32,  60,  72,  63,  43,   2,
    1, -16,  22,  17,  25,  20, -13,  -6,
    -14, -15,  -2,  -5,  -1, -10, -20, -22,
    -30,  -6, -13, -11, -16, -11, -16, -27,
    -36, -18,   0, -19, -15, -15, -21, -38,
    -39, -30, -31, -13, -31, -36, -34, -42,
];

#[rustfmt::skip]
const KING_VALUES: [i16; 64] = [
    4,  54,  47, -99, -99,  60,  83, -62,
    -32,  10,  55,  56,  56,  55,  10,   3,
    -62,  12, -57,  44, -67,  28,  37, -31,
    -55,  50,  11,  -4, -19,  13,   0, -49,
    -55, -43, -52, -28, -51, -47,  -8, -50,
    -47, -42, -43, -79, -64, -32, -29, -32,
    -4,   3, -14, -50, -57, -18,  13,   4,
    17,  30,  -3, -14,   6,  -1,  40,  18,
];

pub trait Value {
    fn value(&self) -> i16;
}

impl Value for Role {
    fn value(&self) -> i16 {
        match self {
            Role::Pawn => 100,
            Role::Knight => 280,
            Role::Bishop => 320,
            Role::Rook => 479,
            Role::Queen => 929,
            Role::King => 0, //TODO should be hitting this. filter these early, in Shakmaty
        }
    }
}

impl Value for Piece {
    fn value(&self) -> i16 {
        match self.color {
            Color::White => self.role.value(),
            Color::Black => self.role.value() * -1,
        }
    }
}

impl Value for Board {
    fn value(&self) -> i16 {
        fn positional_value(square: Square, piece: &Piece) -> i16 {
            let color_square = match piece.color {
                Color::White => square.flip_vertical(),
                Color::Black => square,
            };
            let raw_value = match piece.role {
                Role::Pawn => PAWN_VALUES[color_square.index() as usize],
                Role::Knight => KNIGHT_VALUES[color_square.index() as usize],
                Role::Bishop => BISHOP_VALUES[color_square.index() as usize],
                Role::Queen => QUEEN_VALUES[color_square.index() as usize],
                Role::Rook => ROOK_VALUES[color_square.index() as usize],
                Role::King => KING_VALUES[color_square.index() as usize],
            };
            piece.color.coefficient() as i16 * raw_value
        }
        self.pieces().into_iter().fold(0, |score, (square, piece)| {
            score + piece.value() + positional_value(square, &piece)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use setup::*;

    #[test]
    fn test_pawns() {
        let position = parse_fen("7k/4P3/8/8/8/8/8/7K w - - 0 1");
        assert_eq!(202, position.board().value());
    }

    #[test]
    fn just_kings() {
        let position = parse_fen("7k/8/8/8/8/8/8/7K w - - 0 1");
        assert_eq!(0, position.board().value());
    }

    #[test]
    fn just_kings_white_advantage() {
        let position = parse_fen("7k/8/8/8/8/8/8/6K1 w - - 0 1");
        assert_eq!(22, position.board().value());
    }

    #[test]
    fn test_starting_pos() {
        let position = Chess::default();
        assert_eq!(0, position.board().value());
    }

    #[test]
    fn test_black_up_a_queen() {
        let position = parse_fen("rn1qkbnr/ppp1pppp/8/3p4/3Pb3/8/PPP1PPPP/RNB1KBNR w KQkq - 0 1");
        assert_eq!(-929 + 13 - 41, position.board().value()); //queen value + queen pos + bishop pos diff
    }

    #[test]
    fn test_max_black_advantage() {
        let position = parse_fen("rr1q1q1q/nnk4q/bb5q/7q/8/6K1/2q5/1q1q4 w - - 0 1");
        assert_eq!(-10260, position.board().value());
    }
}
