use std::io::prelude::*;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use structopt::StructOpt;

struct Engine {
    process: std::process::Child,
}

impl Engine {
    fn new(command_name: &str) -> Engine {
        Engine {
            process: match Command::new(command_name)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
            {
                Ok(process) => process,
                Err(err) => panic!("counldn't spawn {}: {}", command_name, err),
            },
        }
    }
    fn input(&mut self, s: &str) {
        let s = s.to_string() + "\n";
        let stdin = self.process.stdin.as_mut().unwrap();
        match stdin.write(s.as_bytes()) {
            Ok(_) => {}
            Err(err) => panic!("coundn't write_all to stdin: {}", err),
        };
        // if stdin dropped and the pipe is closed.
        // but stdin is mutable reference.
        // pipe is not closed.
    }
    fn output(&mut self, start_str: &str) -> String {
        let mut buf = std::io::BufReader::new(self.process.stdout.as_mut().unwrap());
        let mut line = String::new();
        loop {
            buf.read_line(&mut line).unwrap();
            if line.starts_with(start_str) {
                break;
            }
            line.clear();
        }
        line
    }
    fn output_one_line(&mut self) -> String {
        let mut buf = std::io::BufReader::new(self.process.stdout.as_mut().unwrap());
        let mut line = String::new();
        buf.read_line(&mut line).unwrap();
        line
    }
}

struct MatchPair {
    engines: [Engine; 2],
    first_move_engine_index: usize,
    win: Arc<[AtomicUsize; 2]>,
    draw: Arc<AtomicUsize>,
    current_match_num: Arc<AtomicUsize>,
    match_num: usize,
    movetime: usize,
}

impl MatchPair {
    fn start_matches(&mut self) {
        self.engines[0].input("setoption name Threads value 1");
        self.engines[0].input("setoption name USI_Hash value 1024");
        self.engines[0].input("setoption name Byoyomi_Margin value 0");

        self.engines[1].input("setoption name Threads value 1");
        self.engines[1].input("setoption name USI_Hash value 1024");
        self.engines[1].input("setoption name Byoyomi_Margin value 0");

        for engine in self.engines.iter_mut() {
            engine.input("isready");
        }
        for engine in self.engines.iter_mut() {
            engine.output("readyok");
        }

        'next_game: while self.current_match_num.fetch_add(1, Ordering::Relaxed) < self.match_num {
            for engine in self.engines.iter_mut() {
                engine.input("usinewgame");
            }

            let mut sfen = "position startpos moves".to_string();
            self.first_move_engine_index = 1 - self.first_move_engine_index;
            let mut color = self.first_move_engine_index;
            let mut key_hash = std::collections::HashMap::new();
            for _ply in 1..=320 {
                self.engines[color].input(&sfen);
                self.engines[color].input("key");
                let key = self.engines[color].output_one_line();
                *key_hash.entry(key.clone()).or_insert(1) += 1;
                if key_hash[&key] >= 4 {
                    self.draw.fetch_add(1, Ordering::Relaxed);
                    println!(
                        "{}",
                        result_string(
                            self.win[0].load(Ordering::Relaxed),
                            self.win[1].load(Ordering::Relaxed),
                            self.draw.load(Ordering::Relaxed)
                        )
                    );
                    continue 'next_game;
                }
                self.engines[color].input(&format!("go byoyomi {}", self.movetime));
                let bestmove = self.engines[color].output("bestmove");
                let bestmove = bestmove.split_whitespace().collect::<Vec<_>>()[1];
                match bestmove {
                    "win" => {
                        self.win[color].fetch_add(1, Ordering::Relaxed);
                        println!(
                            "{}",
                            result_string(
                                self.win[0].load(Ordering::Relaxed),
                                self.win[1].load(Ordering::Relaxed),
                                self.draw.load(Ordering::Relaxed)
                            )
                        );
                        continue 'next_game;
                    }
                    "resign" => {
                        self.win[1 - color].fetch_add(1, Ordering::Relaxed);
                        println!(
                            "{}",
                            result_string(
                                self.win[0].load(Ordering::Relaxed),
                                self.win[1].load(Ordering::Relaxed),
                                self.draw.load(Ordering::Relaxed)
                            )
                        );
                        continue 'next_game;
                    }
                    m => sfen += &format!(" {}", m),
                }
                color = 1 - color;
            }
            self.draw.fetch_add(1, Ordering::Relaxed);
            println!(
                "{}",
                result_string(
                    self.win[0].load(Ordering::Relaxed),
                    self.win[1].load(Ordering::Relaxed),
                    self.draw.load(Ordering::Relaxed)
                )
            );
        }
    }
}

struct MatchManager {
    match_pairs: Vec<Arc<Mutex<MatchPair>>>,
}

impl MatchManager {
    fn new(
        engine_name_target: &str,
        engine_name_reference: &str,
        parallel_num: usize,
        match_num: usize,
        movetime: usize,
    ) -> MatchManager {
        let win = Arc::new([AtomicUsize::new(0), AtomicUsize::new(0)]);
        let draw = Arc::new(AtomicUsize::new(0));
        let current_match_num = Arc::new(AtomicUsize::new(0));
        let mut match_pairs = vec![];
        for i in 0..parallel_num {
            match_pairs.push(Arc::new(Mutex::new(MatchPair {
                engines: [
                    Engine::new(engine_name_target),
                    Engine::new(engine_name_reference),
                ],
                first_move_engine_index: i % 2,
                win: win.clone(),
                draw: draw.clone(),
                current_match_num: current_match_num.clone(),
                match_num,
                movetime,
            })));
        }
        MatchManager { match_pairs }
    }
    fn start_matches(&mut self) {
        let mut threads = vec![];
        for i in 0..self.match_pairs.len() {
            let match_pair_cloned = self.match_pairs[i].clone();
            match_pair_cloned.lock().unwrap().first_move_engine_index = i % 2;
            threads.push(std::thread::spawn(move || {
                match_pair_cloned.lock().unwrap().start_matches();
            }));
        }
        for thread in threads {
            thread.join().unwrap();
        }
    }
}

fn winning_rate(win: usize, lose: usize, draw: usize) -> f64 {
    let total_mult_2 = (win + lose + draw) * 2;
    let win_mult_2 = win * 2 + draw;
    win_mult_2 as f64 / total_mult_2 as f64
}

fn confidence_interval(win: usize, lose: usize, draw: usize) -> f64 {
    let total = win + lose + draw;
    let win_rate = winning_rate(win, lose, draw);
    1.96 * (win_rate * (1.0 - win_rate) / total as f64).sqrt()
}

fn elo(win: usize, lose: usize, draw: usize) -> Option<f64> {
    if draw == 0 && (win == 0 || lose == 0) {
        None
    } else {
        Some(400.0 * -(1.0 / winning_rate(win, lose, draw) - 1.0).log10())
    }
}

fn result_string(win: usize, lose: usize, draw: usize) -> String {
    format!(
        "({m:>5}) W: {w:>5} L: {l:>5} D: {d:>5} WR: {wr:>6.2}% +-{ci:>6.2}% Elo: {elo:>4}",
        m = win + lose + draw,
        w = win,
        l = lose,
        d = draw,
        wr = 100.0 * winning_rate(win, lose, draw),
        ci = 100.0 * confidence_interval(win, lose, draw),
        elo = match elo(win, lose, draw) {
            Some(n) => (n.round() as i64).to_string(),
            None => "None".to_string(),
        },
    )
}

#[derive(Debug, StructOpt)]
#[structopt(name = "usi_play_match", about = "Play matches and print results.")]
struct Opt {
    /// Target USI engine
    #[structopt(parse(from_os_str))]
    target: PathBuf,
    /// Reference USI engine
    #[structopt(parse(from_os_str))]
    reference: PathBuf,
    /// Parallel Num
    parallel_num: usize,
    /// Match Num
    match_num: usize,
    /// movetime
    movetime: usize,
}

fn main() {
    let opt = Opt::from_args();
    let mut match_manager = MatchManager::new(
        opt.target.as_path().to_str().unwrap(),
        opt.reference.as_path().to_str().unwrap(),
        opt.parallel_num,
        opt.match_num,
        opt.movetime,
    );
    match_manager.start_matches();
}
