use crate::board::Board;

use super::evaluator_trait::Evaluator;

pub struct PriorityEvaluator {}

impl Evaluator for PriorityEvaluator {
    fn get_fitness(&self, board: &Board) -> i64 {
        // let powers: [u8; 16] = [1, 2, 4, 6, 14, 12, 10, 8, 16, 18, 20, 22, 30, 28, 26, 24];
        let powers: [u8; 16] = [2, 1, 0, 0, 4, 2, 1, 0, 6, 8, 10, 12, 20, 18, 16, 14];

        board.board.iter().zip(&powers).fold(0, |acc, (&x, &y)| {
            acc + if x + y > 0 { 1 << (x + y) } else { 0 }
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_get_fitness() {
//         let board = [0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 5, 4, 3, 2];
//         let expected = (1 << 25) + (1 << 22) + (1 << 19) + (1 << 16) + (1 << 3);
//         let fitness = BeamSearch::get_fitness(&board);
//         assert_eq!(fitness, expected);
//     }
// }
