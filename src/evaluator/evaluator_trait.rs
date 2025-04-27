use crate::board::Board;

pub trait Evaluator {
    fn get_fitness(&self, board: &Board) -> i64;
}
