mod agent;
mod board;
mod database;
mod evaluator;

use agent::agent_trait::Agent;
use agent::node::Node;
use agent::{BeamSearch, BeamSearchV2};
use board::Board;
use evaluator::{MetaEvaluator, PriorityEvaluator};

// fn run_seed(seed: u64, db: &mut database::Database) {
//     let game = Board::new(seed);
//     println!("{:#}", game);

//     let start = std::time::Instant::now();
//     let mut ans = Node::new(&game);
//     db.insert(&ans, "BeamSearch", "PriorityEvaluator", seed);

//     let evaluator = PriorityEvaluator {};
//     let beam_search: BeamSearch<PriorityEvaluator> = BeamSearch { evaluator };

//     for _ in 0..600 {
//         ans = beam_search.search_with_depth(&mut ans, 250);
//         // db.insert(&ans, "BeamSearch", "PriorityEvaluator", seed);
//         if ans.board.is_game_over() {
//             break;
//         }
//     }
//     // let ans = beam_search.search(&root);
//     let elapsed = start.elapsed();

//     println!("Actions: {}", ans.action.len());
//     println!("Actions: {}", ans.action);
//     println!("{:#?}", ans.board);
//     println!("Time: {:?}", elapsed.as_millis());
// }

fn eval_seed(seed: u64) -> u32 {
    let game = Board::new(seed);
    let mut ans = Node::new(&game);

    let evaluator = MetaEvaluator {};
    let beam_search: BeamSearch<MetaEvaluator> = BeamSearch { evaluator };

    for _ in 0..600 {
        ans = beam_search.search_with_depth(&mut ans, 250);
        if ans.board.is_game_over() {
            println!("{:#?}", ans.board);
            break;
        }
    }

    // ans = beam_search.search(&mut ans);
    println!("{:#?}", ans.board);
    println!("Seed: {} - Score {}", seed, ans.board.score);

    ans.board.score
}

fn compute_full_score() {
    let seed = vec![
        42, 290797, 10682358, 38333962, 47049887, 11205586, 15242016, 32019767, 46946765, 4424780,
        2524322, 20797492, 28944706, 20969426, 20950077, 8601721, 44677966, 534357, 970088,
        8078305, 5731756, 45283038, 17769313, 41900735, 32506342, 28758123, 25880068, 41359522,
        704563, 29082488,
    ];
    let start = std::time::Instant::now();
    let total: u32 = seed.iter().map(|&s| eval_seed(s)).sum();
    println!("Total: {}", total);
    let elapsed = start.elapsed();
    println!("Time: {:?}", elapsed.as_secs_f32());
}

fn main() {
    eval_seed(290797);
    // compute_full_score();
}

// 17.3M -- step 250 -- 224.74596 s
// 52.2 M -- full
