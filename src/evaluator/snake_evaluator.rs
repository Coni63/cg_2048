use std::cmp::max;

use crate::board::Board;

use super::evaluator_trait::Evaluator;

pub struct SnakeEvaluator {}

impl Evaluator for SnakeEvaluator {
    fn get_fitness(&self, board: &Board) -> u64 {
        let order = [12, 13, 14, 15, 11, 10, 9, 8, 4, 5]; //, 6, 7, 3, 2, 1, 0];

        let mut longest_seq: u64 = 0;
        let mut curr_seq: u64 = 0;
        let mut count_val: u64 = 0;
        let mut prev_val = board.board[order[0]];
        for idx in order.iter() {
            let v = board.board[*idx];
            if v == 0 {
                continue;
            }
            count_val += 1;
            if v <= prev_val {
                curr_seq += 1;
                longest_seq = max(longest_seq, curr_seq);
            } else {
                curr_seq = 1;
            }
            prev_val = v;
        }

        longest_seq * 100 / count_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_evaluator_1() {
        let board = Board {
            board: [3, 0, 4, 0, 8, 7, 6, 5, 10, 0, 8, 10, 16, 15, 14, 12],
            score: 0,
            seed: 0,
        };
        let evaluator = SnakeEvaluator {};

        let fitness = evaluator.get_fitness(&board);
        assert_eq!(fitness, 53);
    }

    #[test]
    fn test_snake_evaluator_2() {
        let board = Board {
            board: [1, 2, 3, 4, 8, 7, 6, 5, 9, 10, 11, 12, 16, 15, 14, 13],
            score: 0,
            seed: 0,
        };
        let evaluator = SnakeEvaluator {};

        let fitness = evaluator.get_fitness(&board);
        assert_eq!(fitness, 100);
    }

    #[test]
    fn test_snake_evaluator_3() {
        let board = Board {
            board: [0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 12, 14, 16, 14, 15, 13],
            score: 0,
            seed: 0,
        };
        let evaluator = SnakeEvaluator {};

        let fitness = evaluator.get_fitness(&board);
        assert_eq!(fitness, 28);
    }
}
