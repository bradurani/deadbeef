use engine::*;
use log::*;
use shakmaty::uci::Uci;
use shakmaty::Color::*;
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
            let mut input = String::new();
            stdin.lock().read_line(&mut input).unwrap();
            self.run_command(engine, &input.trim_right()[..]);
        }
    }

    pub fn run_command(&mut self, engine: &mut Engine, cmd: &str) {
        warn!("RECEIVED: {}", cmd);

        if cmd == "quit" {
            process::exit(0)
        } else if cmd == "protover 2" {
            send("feature done=0");
            send("feature myname=\"deadbeef\"");
            send("feature usermove=1");
            send("feature setboard=1");
            send("feature ping=1");
            send("feature sigint=0");
            send("feature variants=\"normal\"");
            send("feature done=1");
        } else if cmd == "new" {
            engine.reset();
        } else if cmd.starts_with("setboard") {
            match cmd.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
                [_, fen] => match engine.set_board(fen) {
                    Ok(()) => {}
                    Err(msg) => error!("{}", msg),
                },
                _ => error!("invalid setboard {}", cmd),
            }
        } else if cmd == "force" {
            self.force = true;
        } else if cmd == "go" {
            self.force = false;
            go(engine)
        } else if cmd.starts_with("ping") {
            match cmd.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
                [_, n] => send(&format!("pong {}", n)),
                _ => error!("invalid ping {}", cmd),
            }
        } else if cmd.starts_with("usermove") {
            match cmd.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
                [_, action] => match engine.make_user_move(action) {
                    Ok(_action) => {
                        if !engine.game_over() {
                            go(engine);
                        }
                    }
                    Err(msg) => error!("{}", msg),
                },
                _ => error!("invalid usermove {}", cmd),
            }
        } else if cmd.starts_with("time") {
            match cmd.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
                [_, time] => match time.parse::<u64>() {
                    Ok(time_cs) => engine.set_time_remaining_cs(time_cs),
                    Err(msg) => error!("{}", msg),
                },
                _ => error!("invalid time {}", cmd),
            }
        } else if cmd.starts_with("otim") {
            match cmd.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
                [_, time] => match time.parse::<u64>() {
                    Ok(time_cs) => engine.set_opponent_time_remaining_cs(time_cs),
                    Err(msg) => error!("{}", msg),
                },
                _ => error!("invalid time {}", cmd),
            }
        } else if cmd.starts_with("post") {
            engine.set_show_thinking(true);
        } else if cmd.starts_with("nopost") {
            engine.set_show_thinking(false);
        } else if cmd == "white" {
            engine.set_color(White);
        } else if cmd == "black" {
            engine.set_color(Black);
        } else if vec!["xboard", "random", "hard", "accepted", "level"]
            .iter()
            .any(|c| cmd.starts_with(c))
        {

        } else {
            error!("Unknown cmd {}", cmd);
        }
    }
}

fn go(engine: &mut Engine) {
    let best_move = engine.make_engine_move();
    let uci = Uci::from_move(&engine.previous_position, &best_move);
    send(&format!("move {}", uci.to_string()));
}

fn send(msg: &str) {
    println!("{}", msg);
    warn!("SENDING: {}", msg);
}
