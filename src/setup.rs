use shakmaty::{Chess, fen::Fen, Move, uci::Uci};

pub fn parse_fen(fen_str: &'static str) -> Chess {
    let setup: Fen = fen_str.parse().unwrap();
    let position: Chess = setup.position().unwrap();
    position
}

pub fn parse_uci(uci_str: &'static str, position: &Chess) -> Move {
    let uci: Uci = uci_str.parse().unwrap();
    let m = uci.to_move(position).unwrap();
    m
}
