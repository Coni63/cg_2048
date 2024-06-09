mod agent;
mod board;

use agent::agent_trait::Agent;
use agent::beam_search::BeamSearch;
use board::Board;

#[allow(dead_code)]
fn main() {
    let game = Board::new(290797);

    println!("{:#}", game);

    let start = std::time::Instant::now();
    let ans = BeamSearch::search(&game);
    let elapsed = start.elapsed();

    println!("Actions: {}", ans.action);
    println!("{:#?}", ans.board);
    println!("Time: {:?}", elapsed.as_millis());
}
