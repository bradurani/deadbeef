use shakmaty::{fen::Fen, uci::Uci, Chess, Move};

pub fn parse_fen(fen_str: &str) -> Chess {
    let setup: Fen = fen_str.parse().expect("invalid fen");
    setup.position().expect("invalid position")
}

pub fn parse_uci(uci_str: &str, position: &Chess) -> Move {
    let uci: Uci = uci_str.parse().expect("invalid uci");
    uci.to_move(position).expect("invalid position")
}
