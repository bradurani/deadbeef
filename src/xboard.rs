use engine::*;
use shakmaty::uci::Uci;
use std::io::{self, BufRead};

#[derive(Default)]
pub struct XBoard {
    show_thinking: bool,
    force: bool,
}

impl XBoard {
    pub fn start(engine: &mut Engine) {
        let mut xboard: XBoard = Default::default();
        let stdin = io::stdin();

        loop {
            let input = stdin.lock().lines().next().map(|r| r.unwrap());
            match input {
                Some(cmd) => {
                    if cmd == "quit" {
                        break;
                    } else if cmd == "protover 2" {
                        println!("feature done=0");
                        println!("feature myname=\"deadbeef\"");
                        println!("feature usermove=1");
                        println!("feature setboard=1");
                        println!("feature ping=1");
                        println!("feature sigint=0");
                        println!("feature variants=\"normal\"");
                        println!("feature done=1");
                    } else if cmd == "new" {
                        engine.reset();
                    } else if cmd.starts_with("setboard") {
                        match cmd.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
                            [_, fen] => match engine.set_board(fen) {
                                Ok(()) => {}
                                Err(msg) => eprintln!("{}", msg),
                            },
                            _ => eprintln!("invalid setboard {}", cmd),
                        }
                    } else if cmd == "force" {
                        xboard.force = true;
                    } else if cmd == "go" {
                        xboard.force = false;
                        let best_move = engine.make_move();
                        let uci = Uci::from_move(&engine.state.position, &best_move);
                        println!("move {}", uci.to_string());
                    } else {
                        eprintln!("Unknown cmd {}", cmd);
                    }
                }
                None => {}
            }
        }
    }
}
