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
            let result = self.run_command(engine, &input.trim_right()[..]);
            match result {
                Ok(_) => {}
                Err(msg) => error!("{}", msg),
            }
        }
    }

    pub fn run_command(&mut self, engine: &mut Engine, cmd: &str) -> Result<(), String> {
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
            let fen: &str = cmd
                .splitn(2, ' ')
                .collect::<Vec<&str>>()
                .get(1)
                .ok_or("no board string".to_string())?;
            engine.set_board(fen)?;
        } else if cmd == "force" {
            self.force = true;
        } else if cmd == "go" {
            self.force = false;
            go(engine)?;
        } else if cmd.starts_with("ping") {
            let n: &str = cmd
                .splitn(2, ' ')
                .collect::<Vec<&str>>()
                .get(1)
                .ok_or("no ping num".to_string())?;
            send(&format!("pong {}", n));
        } else if cmd.starts_with("usermove") {
            let action: &str = cmd
                .splitn(2, ' ')
                .collect::<Vec<&str>>()
                .get(1)
                .ok_or("no user move".to_string())?;
            engine.make_user_move(action)?;
            if !engine.is_game_over() {
                go(engine)?;
            }
        } else if cmd.starts_with("time") {
            let time: &str = cmd
                .splitn(2, ' ')
                .collect::<Vec<&str>>()
                .get(1)
                .ok_or("missing time".to_string())?;
            let time_cs = time.parse::<u64>().map_err(|e| e.to_string())?;
            engine.set_time_remaining_cs(time_cs);
        } else if cmd.starts_with("otim") {
            let time: &str = cmd
                .splitn(2, ' ')
                .collect::<Vec<&str>>()
                .get(1)
                .ok_or("missing time".to_string())?;
            let time_cs = time.parse::<u64>().map_err(|e| e.to_string())?;
            engine.set_opponent_time_remaining_cs(time_cs);
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

        } else if cmd == "search" {
            // 0xDEADBEEF extensions. Not part of xboard
            engine.search();
        } else if cmd.starts_with("printtree") {
            let args: Vec<&str> = cmd.split(' ').collect();
            let uci_strs: Vec<&str> = args.into_iter().skip(1).collect();
            engine.print_subtree(uci_strs)?;
        } else {
            return Err("unknown command".to_string());
        };
        Ok(())
    }
}

fn go(engine: &mut Engine) -> Result<(), String> {
    let best_move = engine.make_engine_move()?;
    let uci = Uci::from_move(&engine.previous_position, &best_move);
    send(&format!("move {}", uci.to_string()));
    Ok(())
}

fn send(msg: &str) {
    println!("{}", msg);
    warn!("SENDING: {}", msg);
}
