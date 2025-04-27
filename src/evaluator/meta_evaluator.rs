use crate::board::Board;

use super::empty_cell_evaluator::EmptyCellEvaluator;
use super::evaluator_trait::Evaluator;
use super::monotonicity_evaluator::MonotonicityEvaluator;
use super::priority_evaluator::PriorityEvaluator;
use super::smoothness_evaluator::SmoothnessEvaluator;
use super::snake_evaluator::SnakeEvaluator;

pub struct MetaEvaluator {}

impl Evaluator for MetaEvaluator {
    fn get_fitness(&self, board: &Board) -> i64 {
        // let snake_evaluator = SnakeEvaluator {}; // 0 - 100 % of tiles well organised
        let empty_cell_evaluator = EmptyCellEvaluator {}; // absolute max is 16
                                                          // let priority_evaluator = PriorityEvaluator {}; // absolute max is 2^37 < X < 2^38 ~ 150 Md
        let monotonicity_evaluator = MonotonicityEvaluator {};
        let smoothness_evaluator = SmoothnessEvaluator {};

        // let snake_fitness = snake_evaluator.get_fitness(board);
        let empty_cell_fitness = empty_cell_evaluator.get_fitness(board);
        // let priority_fitness = priority_evaluator.get_fitness(board);
        let monotonicity_fitness = monotonicity_evaluator.get_fitness(board);
        let smoothness_fitness = smoothness_evaluator.get_fitness(board);

        // let empty_cell_fitness = 1; //empty_cell_fitness * 3 + 52; // 52 - 100
        // (priority_fitness * snake_fitness * empty_cell_fitness) / 100

        monotonicity_fitness * 5 + smoothness_fitness * 3 + empty_cell_fitness * 2
    }
}

// Seed: 290797 - Score 799832
