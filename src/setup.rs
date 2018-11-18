use emojify::*;
use shakmaty::fen::*;
use shakmaty::uci::*;
use shakmaty::*;

pub fn parse_fen_input(fen_str: &str) -> Result<Chess, String> {
    fen_str
        .parse()
        .map_err(|e: ParseFenError| e.to_string())
        .and_then(|f: Fen| f.position().map_err(|e: PositionError| e.to_string()))
}

pub fn parse_uci_input(uci_str: &str, position: &Chess) -> Result<Move, String> {
    uci_str
        .parse()
        .map_err(|e: ParseUciError| e.to_string())
        .and_then(|uci: Uci| {
            let action = uci.to_move(position);
            action.map_err(|e: IllegalMoveError| e.to_string())
        })
}

pub fn parse_fen(fen_str: &str) -> Chess {
    info!("parsing position:\n{}", fen_str);
    let setup: Fen = fen_str.parse().expect("invalid fen");
    let position: Chess = setup.position().expect("invalid position");
    info_emojified(&position.board());
    position
}

pub fn parse_uci(uci_str: &str, position: &Chess) -> Move {
    let uci: Uci = uci_str.parse().expect("invalid uci");
    uci.to_move(position).expect("invalid position")
}
