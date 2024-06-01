mod agent;
mod board;

use board::Board;

#[allow(dead_code)]
fn main() {
    let game = Board::new(290797);

    println!("{:#}", game);

    let start = std::time::Instant::now();
    let ans = agent::beam_search(&game);
    let elapsed = start.elapsed();

    println!("Actions: {}", ans.action);
    println!("{:#?}", ans.board);
    println!("Time: {:?}", elapsed.as_millis());
}
