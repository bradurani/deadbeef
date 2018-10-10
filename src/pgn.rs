use shakmaty::san::SanPlus;
use shakmaty::*;

pub fn to_pgn(start_position: &Chess, moves: &Vec<Move>) -> String {
    let mut pgn_string = String::new();
    let outcome = push_game_str(start_position.clone(), &moves, &mut pgn_string);
    format!("{}{}", to_result_str(outcome), pgn_string)
}

fn push_game_str(
    mut position: Chess,
    moves: &Vec<Move>,
    pgn_string: &mut String,
) -> Option<Outcome> {
    let mut move_num: u32 = 0;
    for m in moves {
        let san = SanPlus::from_move(position.clone(), &m).to_string();
        position.play_safe(m);
        match position.turn() {
            Color::Black => {
                move_num += 1;
                pgn_string.push_str(&format!("{}. {}", move_num, &san));
            }
            Color::White => {
                pgn_string.push_str(&format!(" {} ", &san));
            }
        }
    }
    position.outcome()
}

fn to_result_str(outcome: Option<Outcome>) -> &'static str {
    match outcome {
        Some(o) => match o {
            Outcome::Draw => "[Result \"1/2-1/2\"]\n",
            Outcome::Decisive {
                winner: Color::White,
            } => "[Result \"1-0\"]\n",
            Outcome::Decisive {
                winner: Color::Black,
            } => "[Result \"0-1\"]\n",
        },
        None => "",
    }
}
