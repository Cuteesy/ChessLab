use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc::{channel, Receiver},
};

/// Latest analysis reported by the engine for the current position.
#[derive(Clone, Debug, Default)]
pub struct Info {
    pub depth: u32,
    pub nodes: u64,
    pub cp: Option<i32>,
    pub mate: Option<i32>,
    pub pv: Vec<String>,
}

impl Info {
    pub fn score_text(&self) -> String {
        if let Some(mate) = self.mate {
            return format!("{}M{}", if mate < 0 { "-" } else { "" }, mate.abs());
        }
        match self.cp {
            Some(cp) => format!("{:+.2}", cp as f64 / 100.0),
            None => "0.00".to_string(),
        }
    }

    /// Centipawns from the side to move's point of view, clamped for the bar.
    pub fn eval_cp(&self) -> f32 {
        if let Some(mate) = self.mate {
            return if mate > 0 { 900.0 } else { -900.0 };
        }
        (self.cp.unwrap_or(0) as f32).clamp(-900.0, 900.0)
    }
}

enum Msg {
    Info(Info),
    Best(String),
}

/// A Stockfish (or any UCI engine) child process with a reader thread.
pub struct Engine {
    child: Child,
    stdin: ChildStdin,
    rx: Receiver<Msg>,
    pub info: Info,
    pub best: Option<String>,
}

impl Engine {
    pub fn spawn(path: &str) -> anyhow::Result<Self> {
        let mut child = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("engine stdin unavailable"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("engine stdout unavailable"))?;

        let (tx, rx) = channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                if let Some(rest) = line.strip_prefix("bestmove ") {
                    if let Some(best) = rest.split_whitespace().next() {
                        let _ = tx.send(Msg::Best(best.to_string()));
                    }
                } else if line.starts_with("info ") && line.contains(" pv ") {
                    let _ = tx.send(Msg::Info(parse_info(&line)));
                }
            }
        });

        let mut engine = Self {
            child,
            stdin,
            rx,
            info: Info::default(),
            best: None,
        };
        engine.command("uci")?;
        engine.command("setoption name Threads value 2")?;
        engine.command("setoption name Hash value 128")?;
        engine.command("isready")?;
        Ok(engine)
    }

    pub fn command(&mut self, cmd: &str) -> anyhow::Result<()> {
        writeln!(self.stdin, "{cmd}")?;
        self.stdin.flush()?;
        Ok(())
    }

    pub fn analyse(&mut self, fen: &str, depth: u32) -> anyhow::Result<()> {
        self.info = Info::default();
        self.best = None;
        self.command("stop")?;
        self.command(&format!("position fen {fen}"))?;
        self.command(&format!("go depth {depth}"))?;
        Ok(())
    }

    /// Drain queued messages without blocking the UI thread.
    pub fn poll(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                Msg::Info(info) => self.info = info,
                Msg::Best(best) => self.best = Some(best),
            }
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        let _ = self.command("quit");
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn parse_info(line: &str) -> Info {
    let mut info = Info::default();
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            "depth" => {
                info.depth = tokens.get(i + 1).and_then(|t| t.parse().ok()).unwrap_or(0);
                i += 2;
            }
            "nodes" => {
                info.nodes = tokens.get(i + 1).and_then(|t| t.parse().ok()).unwrap_or(0);
                i += 2;
            }
            "score" => match tokens.get(i + 1).copied() {
                Some("cp") => {
                    info.cp = tokens.get(i + 2).and_then(|t| t.parse().ok());
                    i += 3;
                }
                Some("mate") => {
                    info.mate = tokens.get(i + 2).and_then(|t| t.parse().ok());
                    i += 3;
                }
                _ => i += 2,
            },
            "pv" => {
                info.pv = tokens[i + 1..].iter().map(|s| s.to_string()).collect();
                break;
            }
            _ => i += 1,
        }
    }
    info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_score_and_pv() {
        let info = parse_info(
            "info depth 18 seldepth 24 multipv 1 score cp -34 nodes 123456 nps 900000 pv e2e4 e7e5 g1f3",
        );
        assert_eq!(info.depth, 18);
        assert_eq!(info.nodes, 123456);
        assert_eq!(info.cp, Some(-34));
        assert_eq!(info.pv, vec!["e2e4", "e7e5", "g1f3"]);
        assert_eq!(info.score_text(), "-0.34");
    }

    #[test]
    fn parses_mate_score() {
        let info = parse_info("info depth 12 score mate 3 nodes 42 pv d1h5 g8f6");
        assert_eq!(info.mate, Some(3));
        assert_eq!(info.score_text(), "M3");
        assert!(info.eval_cp() > 0.0);
    }
}
