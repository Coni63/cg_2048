use crate::board::Board;

use super::evaluator_trait::Evaluator;

pub struct EmptyCellEvaluator {}

impl Evaluator for EmptyCellEvaluator {
    fn get_fitness(&self, board: &Board) -> u64 {
        board.board.iter().filter(|&x| *x == 0).count() as u64
    }
}
