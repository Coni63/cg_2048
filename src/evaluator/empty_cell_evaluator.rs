use crate::board::Board;

use super::evaluator_trait::Evaluator;

pub struct EmptyCellEvaluator {}

impl Evaluator for EmptyCellEvaluator {
    // this score is between 0 and 16
    // 0 - no empty cells
    // 16 - all cells are empty
    fn get_fitness(&self, board: &Board) -> i64 {
        let c = board.board.iter().filter(|&x| *x == 0).count() as i64;

        if c == 0 {
            -100
        } else {
            c
        }
    }
}
