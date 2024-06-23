use crate::board::Board;

use super::empty_cell_evaluator::EmptyCellEvaluator;
use super::evaluator_trait::Evaluator;
use super::monoticity_evaluator::MonoticityEvaluator;
use super::priority_evaluator::PriorityEvaluator;

pub struct MetaEvaluator {}

impl Evaluator for MetaEvaluator {
    fn get_fitness(&self, board: &Board) -> u64 {
        let monoticity_evaluator = MonoticityEvaluator {}; // 0 - 100 % of tiles well organised
        let empty_cell_evaluator = EmptyCellEvaluator {}; // absolute max is 16
        let priority_evaluator = PriorityEvaluator {}; // absolute max is 2^37 < X < 2^38 ~ 150 Md

        let monoticity_fitness = monoticity_evaluator.get_fitness(board);
        let empty_cell_fitness = empty_cell_evaluator.get_fitness(board);
        let priority_fitness = priority_evaluator.get_fitness(board);

        let empty_cell_fitness = empty_cell_fitness * 3 + 52; // 0 - 100
        (priority_fitness * monoticity_fitness * empty_cell_fitness) / 100
    }
}

// Seed: 290797 - Score 799832
