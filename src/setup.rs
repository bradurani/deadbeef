use shakmaty::{fen::Fen, uci::Uci, Chess, Move};

pub fn parse_fen(fen_str: &'static str) -> Chess {
    let setup: Fen = fen_str.parse().unwrap();
    setup.position().unwrap()
}

pub fn parse_uci(uci_str: &'static str, position: &Chess) -> Move {
    let uci: Uci = uci_str.parse().unwrap();
    uci.to_move(position).unwrap()
}
