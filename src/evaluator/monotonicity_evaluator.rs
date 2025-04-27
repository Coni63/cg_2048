use super::evaluator_trait::Evaluator;
use crate::board::Board;

pub struct MonotonicityEvaluator {}

impl Evaluator for MonotonicityEvaluator {
    // this score is between 0 and 24
    // 0 - no monotonicity
    // 24 - perfect monotonicity
    fn get_fitness(&self, board: &Board) -> i64 {
        let mut total_monotonicity = 0;

        // Check horizontal monotonicity
        for row in 0..4 {
            let mut left_to_right = 0;
            let mut right_to_left = 0;

            let mut last_non_zero_val = 0;
            let mut found_non_zero = false;

            // Left to right
            for col in 0..4 {
                let current_idx = row * 4 + col;
                let current_val = board.board[current_idx];

                if current_val != 0 {
                    if found_non_zero {
                        // Compare with the last non-zero value
                        if current_val >= last_non_zero_val {
                            left_to_right += 1;
                        } else {
                            left_to_right -= 1;
                        }
                    }
                    found_non_zero = true;
                    last_non_zero_val = current_val;
                }
            }

            // Right to left
            found_non_zero = false;
            last_non_zero_val = 0;

            for col in (0..4).rev() {
                let current_idx = row * 4 + col;
                let current_val = board.board[current_idx];

                if current_val != 0 {
                    if found_non_zero {
                        // Compare with the last non-zero value
                        if current_val >= last_non_zero_val {
                            right_to_left += 1;
                        } else {
                            right_to_left -= 1;
                        }
                    }
                    found_non_zero = true;
                    last_non_zero_val = current_val;
                }
            }

            // Take the maximum of the two directions (or 0 if both are negative)
            total_monotonicity += left_to_right.max(right_to_left).max(0);
        }

        // Check vertical monotonicity
        for col in 0..4 {
            let mut top_to_bottom = 0;
            let mut bottom_to_top = 0;

            let mut last_non_zero_val = 0;
            let mut found_non_zero = false;

            // Top to bottom
            for row in 0..4 {
                let current_idx = row * 4 + col;
                let current_val = board.board[current_idx];

                if current_val != 0 {
                    if found_non_zero {
                        // Compare with the last non-zero value
                        if current_val >= last_non_zero_val {
                            top_to_bottom += 1;
                        } else {
                            top_to_bottom -= 1;
                        }
                    }
                    found_non_zero = true;
                    last_non_zero_val = current_val;
                }
            }

            // Bottom to top
            found_non_zero = false;
            last_non_zero_val = 0;

            for row in (0..4).rev() {
                let current_idx = row * 4 + col;
                let current_val = board.board[current_idx];

                if current_val != 0 {
                    if found_non_zero {
                        // Compare with the last non-zero value
                        if current_val >= last_non_zero_val {
                            bottom_to_top += 1;
                        } else {
                            bottom_to_top -= 1;
                        }
                    }
                    found_non_zero = true;
                    last_non_zero_val = current_val;
                }
            }

            // Take the maximum of the two directions (or 0 if both are negative)
            total_monotonicity += top_to_bottom.max(bottom_to_top).max(0);
        }

        total_monotonicity as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monotonicity_perfect() {
        // A perfectly monotonic board (increasing from top-left to bottom-right)
        // Values representing powers of 2: 2^1=2, 2^2=4, 2^3=8, 2^4=16, etc.
        let board = Board {
            board: [1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6, 4, 5, 6, 7],
            score: 0,
            seed: 0,
        };
        let evaluator = MonotonicityEvaluator {};

        let fitness = evaluator.get_fitness(&board);
        assert_eq!(fitness, 24); // Maximum score: 12 horizontal + 12 vertical
    }

    #[test]
    fn test_monotonicity_perfect_reversed() {
        // A perfectly monotonic board (decreasing from bottom-right to top-left)
        let board = Board {
            board: [7, 6, 5, 4, 6, 5, 4, 3, 5, 4, 3, 2, 4, 3, 2, 1],
            score: 0,
            seed: 0,
        };
        let evaluator = MonotonicityEvaluator {};

        let fitness = evaluator.get_fitness(&board);
        assert_eq!(fitness, 24); // Maximum score: 12 horizontal + 12 vertical
    }

    #[test]
    fn test_monotonicity_with_zeros() {
        // A board with some zeros but still good monotonicity
        let board = Board {
            board: [1, 2, 3, 4, 0, 3, 4, 5, 3, 0, 5, 6, 4, 5, 0, 7],
            score: 0,
            seed: 0,
        };

        let evaluator = MonotonicityEvaluator {};

        let fitness = evaluator.get_fitness(&board);
        // We skip comparisons with zeros, so this will have a lower score
        assert_eq!(fitness, 18);
    }

    #[test]
    fn test_monotonicity_mixed() {
        // A mixed board with some monotonicity but not perfect
        let board = Board {
            board: [3, 0, 4, 0, 8, 7, 6, 5, 10, 0, 8, 10, 16, 15, 14, 12],
            score: 0,
            seed: 0,
        };
        let evaluator = MonotonicityEvaluator {};

        let fitness = evaluator.get_fitness(&board);
        // This should have decent but not perfect monotonicity
        assert_eq!(fitness, 16);
    }
}
