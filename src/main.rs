mod agent;
mod board;
mod database;
mod evaluator;

use agent::agent_trait::Agent;
use agent::beam_search::BeamSearch;
use agent::node::Node;
use board::Board;
use evaluator::basic_evaluator::BasicEvaluator;
use evaluator::priority_evaluator::PriorityEvaluator;

fn main() {
    let mut db = database::Database::new("2048.db".to_string());

    let game = Board::new(290797);
    let mut root = Node::new(&game);

    db.insert(&root, "BeamSearch", "PriorityEvaluator");

    if let Some(node) = db.select(1) {
        println!("{:?}", node.board);
    }

    // println!("{:#}", game);

    // let start = std::time::Instant::now();
    // let evaluator = PriorityEvaluator {};
    // let beam_search: BeamSearch<PriorityEvaluator> = BeamSearch { evaluator };
    // let ans = beam_search.search(&root);
    // let ans = beam_search.search_with_depth(&root, 30000);
    // let elapsed = start.elapsed();

    // println!("Actions: {}", ans.action.len());
    // println!("Actions: {}", ans.action);
    // println!("{:#?}", ans.board);
    // println!("Time: {:?}", elapsed.as_millis());
}
