use crate::agent::node::Node;
use crate::board::Board;
use crate::evaluator::basic_evaluator::BasicEvaluator;

pub trait Agent {
    fn new(evaluator: BasicEvaluator) -> Self;
    fn search(&self, board: &Board) -> Node;
}
