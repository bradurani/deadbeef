use shakmaty::uci::*;
use shakmaty::*;
use std::io::{self, BufRead};

pub fn stdin(position: &Chess) -> shakmaty::Move {
    loop {
        let stdin = io::stdin();
        let line = stdin.lock().lines().next();
        match line {
            Some(o) => match o {
                Ok(s) => {
                    let uci_result: Result<Uci, InvalidUci> = s.parse();
                    match uci_result {
                        Ok(uci) => {
                            let action_result = uci.to_move(position);
                            match action_result {
                                Ok(action) => return action,
                                Err(e) => println!("{}", e),
                            }
                        }
                        Err(e) => println!("{}", e),
                    }
                }
                Err(e) => println!("{}", e),
            },
            None => println!("{}", "e"),
        }
    }
}
