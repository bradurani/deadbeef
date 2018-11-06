use state::*;
use std::io::{self, BufRead};

#[derive(Default)]
pub struct XBoard {
    state: State,
    show_thinking: bool,
}

impl XBoard {
    pub fn start() {
        let mut xboard: XBoard = Default::default();

        loop {
            let stdin = io::stdin();
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
                        xboard.state = Default::default();
                    } else if cmd.starts_with("setboard") {
                        match cmd.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
                            [_, fen] => match State::from_fen(fen) {
                                Ok(state) => xboard.state = state,
                                Err(msg) => eprintln!("{}", msg),
                            },
                            _ => eprintln!("invalid setboard {}", cmd),
                        }
                    } else {
                        eprintln!("Unknown cmd {}", cmd);
                    }
                }
                None => {}
            }
        }
    }
}
