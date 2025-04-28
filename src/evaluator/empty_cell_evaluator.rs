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

    fn get_highest_possible_fitness(&self, board: &Board) -> i64 {
        (16u32
            - board
                .board
                .iter()
                .filter(|&&x| x > 0)
                .fold(0u32, |seen, &x| seen | (1 << x))
                .count_ones()) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn test_empty_cell_evaluator() {
        let mut board = Board::new(0);
        board.board = [0, 0, 4, 4, 0, 2, 2, 4, 0, 0, 2, 2, 0, 0, 0, 0];

        let evaluator = EmptyCellEvaluator {};
        assert_eq!(evaluator.get_fitness(&board), 9);
        assert_eq!(evaluator.get_highest_possible_fitness(&board), 14);
    }

    #[test]
    fn test_good_evaluator() {
        let mut board = Board::new(0);
        board.board = [0, 0, 0, 0, 1, 2, 3, 4, 8, 7, 6, 5, 9, 10, 11, 12];

        let evaluator = EmptyCellEvaluator {};
        assert_eq!(evaluator.get_fitness(&board), 4);
        assert_eq!(evaluator.get_highest_possible_fitness(&board), 4);
    }
}
