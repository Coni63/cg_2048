use crate::board::Board;

use super::evaluator_trait::Evaluator;

pub struct BasicEvaluator {}

impl Evaluator for BasicEvaluator {
    fn get_fitness(&self, board: &Board) -> u64 {
        board.score as u64
    }
}
