mod beam;
mod engine;
mod eval;
mod sim;

use std::io::{self, BufRead, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use beam::{Agent, BeamParams};
use engine::Tables;
use eval::{EvalParams, Evaluator};

const MAX_TURNS: usize = 600;

const TEST_SEEDS: [u64; 30] = [
    42, 290797, 10682358, 38333962, 47049887, 11205586, 15242016, 32019767, 46946765, 4424780,
    2524322, 20797492, 28944706, 20969426, 20950077, 8601721, 44677966, 534357, 970088, 8078305,
    5731756, 45283038, 17769313, 41900735, 32506342, 28758123, 25880068, 41359522, 704563,
    29082488,
];

fn default_params() -> (BeamParams, EvalParams) {
    (BeamParams::default(), EvalParams::default())
}

fn codingame() {
    let process_start = Instant::now();
    let (beam_params, eval_params) = default_params();
    let tables = Tables::new();
    let evaluator = Evaluator::new(eval_params);
    let mut agent = Agent::new(&tables, &evaluator, beam_params);
    eprintln!("init {:.1} ms", process_start.elapsed().as_secs_f64() * 1000.0);

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut first = true;
    while let Some(Ok(line)) = lines.next() {
        let start = if first { process_start } else { Instant::now() };
        first = false;
        let seed: u64 = line.trim().parse().unwrap();
        let _score = lines.next();
        let mut cells = [0u8; 16];
        for r in 0..4 {
            let line = lines.next().unwrap().unwrap();
            for (c, v) in line.split_whitespace().enumerate() {
                let v: u32 = v.parse().unwrap();
                cells[4 * r + c] = if v == 0 { 0 } else { v.trailing_zeros() as u8 };
            }
        }
        let out = agent.turn(seed, cells, start);
        let s = &agent.stats;
        eprintln!(
            "{:.1} ms | layers {} width {} played {} frontier {}{}",
            start.elapsed().as_secs_f64() * 1000.0,
            s.layers,
            s.width,
            s.played,
            s.frontier,
            if s.resync { " RESYNC" } else { "" }
        );
        println!("{}", out);
        io::stdout().flush().unwrap();
    }
}

struct GameResult {
    seed: u64,
    score: u32,
    max_tile: u8,
    moves: usize,
    turns: usize,
    finished: bool,
    invalid: usize,
    max_turn_ms: f64,
    late_turns: usize,
    total_ms: f64,
    resyncs: usize,
}

fn sim_action(c: char) -> u8 {
    match c {
        'U' => 1,
        'L' => 2,
        'D' => 3,
        _ => 4,
    }
}

fn run_game(seed: u64, tables: &Tables, evaluator: &Evaluator, params: &BeamParams) -> GameResult {
    let mut game = sim::Board::new(seed);
    let mut agent = Agent::new(tables, evaluator, params.clone());
    let mut res = GameResult {
        seed,
        score: 0,
        max_tile: 0,
        moves: 0,
        turns: 0,
        finished: false,
        invalid: 0,
        max_turn_ms: 0.0,
        late_turns: 0,
        total_ms: 0.0,
        resyncs: 0,
    };
    while res.turns < MAX_TURNS {
        if game.is_game_over() {
            res.finished = true;
            break;
        }
        let start = Instant::now();
        let out = agent.turn(game.seed, game.board, start);
        let ms = start.elapsed().as_secs_f64() * 1000.0;
        let limit = if res.turns == 0 { 1000.0 } else { 50.0 };
        if ms > limit {
            res.late_turns += 1;
            if std::env::var("DEBUG_LATE").is_ok() {
                eprintln!("late: seed {} turn {} {:.1} ms {:?}", seed, res.turns, ms, agent.stats);
            }
        }
        if res.turns > 0 {
            res.max_turn_ms = res.max_turn_ms.max(ms);
        }
        res.total_ms += ms;
        res.resyncs += agent.stats.resync as usize;
        res.turns += 1;
        for c in out.chars() {
            if !game.play(sim_action(c)) {
                res.invalid += 1;
                break;
            }
            res.moves += 1;
        }
    }
    if game.is_game_over() {
        res.finished = true;
    }
    res.score = game.score;
    res.max_tile = *game.board.iter().max().unwrap();
    res
}

fn usage() -> ! {
    eprintln!(
        "usage: cg_2048 bench [--threads N] [--seeds a,b,..] [--count N]\n\
         \x20 [--time MS] [--first MS] [--fixed] [--width W] [--horizon H] [--layers L] [--up]\n\
         \x20 [--e name=value ...]   (eval params: snake empty merges mono mono_pow sum sum_pow)"
    );
    std::process::exit(1)
}

fn bench(args: &[String]) {
    let (mut beam_params, mut eval_params) = default_params();
    let mut threads = 6;
    let mut seeds: Vec<u64> = TEST_SEEDS.to_vec();
    let mut i = 0;
    while i < args.len() {
        let next = |i: usize| -> &String { args.get(i + 1).unwrap_or_else(|| usage()) };
        let num = |i: usize| -> f64 { next(i).parse().unwrap_or_else(|_| usage()) };
        match args[i].as_str() {
            "--threads" => threads = num(i) as usize,
            "--seeds" => seeds = next(i).split(',').map(|s| s.parse().unwrap()).collect(),
            "--count" => seeds.truncate(num(i) as usize),
            "--time" => beam_params.time_ms = num(i),
            "--first" => beam_params.first_time_ms = num(i),
            "--width" => beam_params.width = num(i) as usize,
            "--horizon" => beam_params.horizon = num(i) as usize,
            "--layers" => beam_params.layers_per_turn = num(i) as usize,
            "--fixed" => {
                beam_params.time_ms = 0.0;
                i -= 1;
            }
            "--up" => {
                beam_params.always_up = true;
                i -= 1;
            }
            "--e" => {
                let (name, value) = next(i).split_once('=').unwrap_or_else(|| usage());
                if !eval_params.set(name, value.parse().unwrap_or_else(|_| usage())) {
                    usage();
                }
            }
            _ => usage(),
        }
        i += 2;
    }
    eprintln!("{:?}\n{:?}", beam_params, eval_params);

    let start = Instant::now();
    let tables = Tables::new();
    let evaluator = Evaluator::new(eval_params);
    eprintln!("init {:.0} ms", start.elapsed().as_secs_f64() * 1000.0);

    let next = AtomicUsize::new(0);
    let results = Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for _ in 0..threads.max(1) {
            scope.spawn(|| loop {
                let k = next.fetch_add(1, Ordering::Relaxed);
                if k >= seeds.len() {
                    break;
                }
                let r = run_game(seeds[k], &tables, &evaluator, &beam_params);
                println!(
                    "seed {:>9} score {:>8} tile {:>6} moves {:>6} turns {:>3}{}{} | max {:.1} ms late {} total {:.1} s{}",
                    r.seed,
                    r.score,
                    1u32 << r.max_tile,
                    r.moves,
                    r.turns,
                    if r.finished { "" } else { " CAPPED" },
                    if r.invalid > 0 { " INVALID" } else { "" },
                    r.max_turn_ms,
                    r.late_turns,
                    r.total_ms / 1000.0,
                    if r.resyncs > 1 { " RESYNC" } else { "" },
                );
                results.lock().unwrap().push(r);
            });
        }
    });
    let results = results.into_inner().unwrap();
    let total: u64 = results.iter().map(|r| r.score as u64).sum();
    let capped = results.iter().filter(|r| !r.finished).count();
    println!(
        "TOTAL {} ({:.2}M) over {} games, avg {:.0}, capped {}, wall {:.1} s",
        total,
        total as f64 / 1e6,
        results.len(),
        total as f64 / results.len() as f64,
        capped,
        start.elapsed().as_secs_f64()
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "bench" {
        bench(&args[2..]);
    } else {
        codingame();
    }
}
