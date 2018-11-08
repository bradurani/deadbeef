use engine::*;
use shakmaty::uci::Uci;
use std::io::{self, BufRead};
use std::process;

#[derive(Debug)]
pub struct XBoard {
    force: bool,
}

impl Default for XBoard {
    fn default() -> XBoard {
        XBoard { force: false }
    }
}

impl XBoard {
    pub fn start(&mut self, engine: &mut Engine) {
        let stdin = io::stdin();

        loop {
            let input = stdin.lock().lines().next().map(|r| r.unwrap());
            match input {
                Some(cmd) => self.run_command(engine, &cmd),
                None => eprintln!("invalid input"),
            }
        }
    }

    pub fn run_command(&mut self, engine: &mut Engine, cmd: &str) {
        if cmd == "quit" {
            process::exit(0)
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
            self.force = true;
        } else if cmd == "go" {
            self.force = false;
            let best_move = engine.make_engine_move();
            let uci = Uci::from_move(&engine.previous_position, &best_move);
            println!("move {}", uci.to_string());
        } else if cmd.starts_with("ping") {
            match cmd.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
                [_, n] => println!("pong {}", n),
                _ => eprintln!("invalid ping {}", cmd),
            }
        } else if cmd.starts_with("usermove") {
            match cmd.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
                [_, action] => match engine.make_user_move(action) {
                    Ok(_action) => {
                        if !self.force {
                            self.run_command(engine, "go")
                        }
                    }
                    Err(msg) => eprintln!("{}", msg),
                },
                _ => eprintln!("invalid usermove {}", cmd),
            }
        } else if cmd.starts_with("time") {
            match cmd.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
                [_, time] => match time.parse::<u64>() {
                    Ok(time_cs) => engine.set_time_remaining_cs(time_cs),
                    Err(msg) => eprintln!("{}", msg),
                },
                _ => eprintln!("invalid time {}", cmd),
            }
        } else if cmd.starts_with("otim") {
            match cmd.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
                [_, time] => match time.parse::<u64>() {
                    Ok(time_cs) => engine.set_opponent_time_remaining_cs(time_cs),
                    Err(msg) => eprintln!("{}", msg),
                },
                _ => eprintln!("invalid time {}", cmd),
            }
        } else if cmd.starts_with("post") {
            engine.set_show_thinking(true);
        } else if cmd.starts_with("nopost") {
            engine.set_show_thinking(false);
        } else if vec!["xboard", "random", "hard", "accepted", "level"]
            .iter()
            .any(|c| cmd.starts_with(c))
        {

        } else {
            eprintln!("Unknown cmd {}", cmd);
        }
    }
}
