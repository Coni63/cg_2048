mod agent;
mod board;
mod evaluator;

use agent::agent_trait::Agent;
use agent::beam_search::BeamSearch;
use board::Board;
use evaluator::basic_evaluator::BasicEvaluator;

#[allow(dead_code)]
fn main() {
    let game = Board::new(290797);

    println!("{:#}", game);

    let start = std::time::Instant::now();
    let evaluator = BasicEvaluator {};
    let beam_search: BeamSearch = BeamSearch::new(evaluator);
    let ans = beam_search.search(&game);
    let elapsed = start.elapsed();

    println!("Actions: {}", ans.action);
    println!("{:#?}", ans.board);
    println!("Time: {:?}", elapsed.as_millis());
}
