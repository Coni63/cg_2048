use crate::board::Board;

use super::evaluator_trait::Evaluator;

pub struct PriorityEvaluator {
    powers: [u8; 16],
    sorted_powers: [u8; 16],
}

impl PriorityEvaluator {
    pub fn new() -> Self {
        let mut result = Self {
            // powers: [2, 1, 0, 0, 4, 2, 1, 0, 6, 8, 10, 12, 20, 18, 16, 14],
            powers: [1, 2, 4, 6, 14, 12, 10, 8, 16, 18, 20, 22, 30, 28, 26, 24],
            sorted_powers: [0; 16],
        };
        result.sorted_powers.copy_from_slice(&result.powers);
        result.sorted_powers.sort();
        result
    }

    fn get_score(&self, array: &[u8; 16], powers: &[u8; 16]) -> i64 {
        array.iter().zip(powers).fold(0, |acc, (&x, &y)| {
            acc + if x > 0 && y > 0 { 1 << (x + y) } else { 0 }
        })
    }
}

impl Evaluator for PriorityEvaluator {
    fn get_fitness(&self, board: &Board) -> i64 {
        self.get_score(&board.board, &self.powers)
    }

    fn get_highest_possible_fitness(&self, board: &Board) -> i64 {
        let mut board_copy = board.board;
        board_copy.sort();

        self.get_score(&board_copy, &self.sorted_powers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fitness() {
        let board = Board {
            board: [0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 5, 4, 3, 2],
            score: 0,
            seed: 0,
        };
        let expected = (1 << 25) + (1 << 22) + (1 << 19) + (1 << 16) + (1 << 3);
        let evaluator = PriorityEvaluator::new();
        let fitness = evaluator.get_fitness(&board);
        assert_eq!(fitness, expected);

        let max_expected = (1 << 25) + (1 << 22) + (1 << 19) + (1 << 16) + (1 << 13) + (1 << 11);
        let max_fitness = evaluator.get_highest_possible_fitness(&board);
        assert_eq!(max_fitness, max_expected);
    }

    #[test]
    fn debug_strange_board() {
        let board = Board {
            board: [3, 4, 3, 2, 3, 5, 6, 5, 5, 6, 7, 8, 8, 12, 8, 3],
            score: 0,
            seed: 0,
        };
        let evaluator = PriorityEvaluator::new();

        let fitness = evaluator.get_fitness(&board);
        let max_fitness = evaluator.get_highest_possible_fitness(&board);

        println!("Fitness: {}", fitness);
        println!("Max Fitness: {}", max_fitness);

        println!("Fitness i32: {}", fitness);
        println!("Max Fitness i32: {}", max_fitness as i32);

        let result = fitness as f64 / max_fitness as f64;

        assert!(result <= 1.0);
        assert!(result >= 0.0);
    }
}
