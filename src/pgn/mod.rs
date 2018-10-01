extern crate shakmaty;
use shakmaty::{Chess, Move, MoveList, Outcome, Position, Role, Square, Color, san::San};

pub struct PgnMove {
    white_ply: Move,
    white_position: Chess,
    black_ply: Option<Move>,
    black_position: Chess
}

pub fn to_pgn(moves: Vec<Move>, outcome: Option<Outcome>) -> String{
    let mut s = String::new();
    let result = result_str(outcome);
    s.push_str(result);
    let moves = moves_str(moves);
    s.push_str(moves.as_str());
    s
}

fn result_str(outcome: Option<Outcome>) -> &'static str{
    match outcome {
        Some(o) => match o {
            Outcome::Draw => "[Result \"1/2-1/2\"]\n",
            Outcome::Decisive { winner: Color::White } => "[Result \"1-0\"]\n",
            Outcome::Decisive { winner: Color::Black } => "[Result \"0-1\"]\n"
        },
        None => ""
    }
}

fn moves_str(moves: Vec<Move>) -> String{
    let mut pgn_list: Vec<PgnMove> = Vec::new();
    let mut iter = moves.iter();
    let mut white_pos = Chess::default();
    while let Some(white) = iter.next(){
        let black_pos = white_pos.clone().play(white).unwrap();
        let black_ply = iter.next().map(|b| {
            white_pos = black_pos.clone().play(b).unwrap();
            b.clone()
        });
        pgn_list.push(PgnMove {
            white_ply: white.clone(),
            black_ply: black_ply,
            white_position: white_pos.clone(),
            black_position: black_pos.clone()
        });
    }
    let mut pgn_string: String = String::new();
    for (i, p) in pgn_list.iter().enumerate() {
        pgn_string.push_str(move_str(i, p).as_str());
    }
    pgn_string
}

fn move_str(num: usize, pgn_move: &PgnMove) -> String{
    let white_move_string = San::from_move(&pgn_move.white_position, 
                                               &pgn_move.white_ply).to_string();
    match pgn_move.black_ply {
        None => format!("{}. {}\n", num, white_move_string),
        Some(ref bp) => {
            let black_move_string = San::from_move(&pgn_move.black_position, &bp).to_string();
            format!("{}. {} {}\n", num, white_move_string, black_move_string)
        }
    }.clone()
}
