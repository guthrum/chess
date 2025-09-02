use crate::core::Move;
use anyhow::Context;
use std::fmt;
use std::io::{Read, Write};
use std::process::Child;
use std::time::Duration;

pub struct UciEngine {
    child: Child,
}

impl Default for UciEngine {
    fn default() -> Self {
        Self::new("stockfish").expect("could not start stockfish binary")
    }
}

impl UciEngine {
    pub fn new(stockfish_exec: &str) -> anyhow::Result<Self> {
        let child = std::process::Command::new(stockfish_exec)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to start Stockfish engine at {}", stockfish_exec))?;
        let mut res = Self { child };

        res.command(format_args!("uci\n"), Duration::from_millis(100))?;

        Ok(res)
    }

    pub fn set_option(&mut self, name: &str, value: &str) -> anyhow::Result<()> {
        self.command_without_response(format_args!("setoption name {} value {}\n", name, value))
    }

    pub fn skill_level(&mut self, level: u8) -> anyhow::Result<()> {
        if level > 20 {
            return Err(anyhow::anyhow!("Skill level must be between 0 and 20"));
        }

        self.set_option("Skill Level", &level.to_string())
    }

    pub fn is_ready(&mut self) -> anyhow::Result<()> {
        self.command(format_args!("isready\n"), Duration::from_millis(100))?;
        Ok(())
    }

    pub fn position(&mut self, fen: &str) -> anyhow::Result<()> {
        self.command_without_response(format_args!("position fen {}\n", fen))?;
        Ok(())
    }

    pub fn new_game(&mut self) -> anyhow::Result<()> {
        self.command_without_response(format_args!("ucinewgame\n"))?;
        Ok(())
    }

    pub fn best_move(&mut self) -> anyhow::Result<Move> {
        let res = self.command(
            format_args!("go movetime 1000\n"),
            Duration::from_millis(1050),
        )?;
        // res = bestmove c2c4 ponder e7e5
        res.strip_prefix("bestmove ")
            .and_then(|s| s.split_whitespace().next())
            .and_then(|mv| Move::from_uci(mv).ok())
            .ok_or_else(|| {
                anyhow::anyhow!("Failed to parse best move from Stockfish response: {}", res)
            })
    }

    fn command_without_response(&mut self, command: fmt::Arguments) -> anyhow::Result<()> {
        self.child.stdin.as_mut().unwrap().write_fmt(command)?;
        tracing::debug!("Sent command to Stockfish: {}", command);
        Ok(())
    }

    fn command(&mut self, command: fmt::Arguments, duration: Duration) -> anyhow::Result<String> {
        self.command_without_response(command)?;

        std::thread::sleep(duration);

        tracing::debug!("Reading response from Stockfish...");
        let mut output = vec![];
        loop {
            let line = self.read_line()?;
            tracing::debug!("Read from Stockfish: {}", line);
            match line.trim() {
                "readyok" => break,
                "uciok" => break,
                v => output.push(v.to_string()),
            }
        }
        let res = output.join("\n");

        tracing::debug!("Received response from Stockfish: {}", res);
        Ok(res)
    }

    fn read_line(&mut self) -> anyhow::Result<String> {
        let mut output = String::new();
        loop {
            let mut buf = [0; 1];
            let n = self
                .child
                .stdout
                .as_mut()
                .unwrap()
                .read(&mut buf)
                .with_context(|| "Failed to read from Stockfish stdout")?;
            if n == 0 {
                break;
            }
            let c = buf[0] as char;
            if c == '\n' {
                break;
            }
            output.push(c);
        }
        Ok(output)
    }
}

impl Drop for UciEngine {
    fn drop(&mut self) {
        self.command(format_args!("quit\n"), Duration::from_millis(100))
            .ok();
    }
}
