use super::evaluator_trait::Evaluator;
use crate::board::Board;

pub struct SmoothnessEvaluator {}

impl Evaluator for SmoothnessEvaluator {
    // this score is between 0 and 72
    // 0 - no smoothness
    // 72 - perfect smoothness

    fn get_fitness(&self, board: &Board) -> u64 {
        let mut smoothness = 0;

        // Check horizontal smoothness
        for row in 0..4 {
            for col in 0..3 {
                let current_idx = row * 4 + col;
                let next_idx = current_idx + 1;

                let current_val = board.board[current_idx];
                let next_val = board.board[next_idx];

                // Skip if either cell is empty
                if current_val != 0 && next_val != 0 {
                    // Calculate difference between adjacent tiles
                    // For perfect smoothness, difference should be 0 (same values)
                    let diff = (current_val as i8 - next_val as i8).abs();

                    // Award points for smoothness - smaller differences are better
                    // We use a negative penalty that gets worse with larger differences
                    if diff == 0 {
                        // Perfect match - tiles can be merged!
                        smoothness += 3;
                    } else if diff == 1 {
                        // Close match - only one step away
                        smoothness += 1;
                    }
                    // No points for larger differences
                }
            }
        }

        // Check vertical smoothness
        for col in 0..4 {
            for row in 0..3 {
                let current_idx = row * 4 + col;
                let next_idx = current_idx + 4;

                let current_val = board.board[current_idx];
                let next_val = board.board[next_idx];

                // Skip if either cell is empty
                if current_val != 0 && next_val != 0 {
                    // Calculate difference between adjacent tiles
                    let diff = (current_val as i8 - next_val as i8).abs();

                    if diff == 0 {
                        // Perfect match - tiles can be merged!
                        smoothness += 3;
                    } else if diff == 1 {
                        // Close match - only one step away
                        smoothness += 1;
                    }
                    // No points for larger differences
                }
            }
        }

        smoothness as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_smoothness() {
        // A board with many adjacent tiles of the same value
        let board = Board {
            board: [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            score: 0,
            seed: 0,
        };
        let evaluator = SmoothnessEvaluator {};

        let fitness = evaluator.get_fitness(&board);
        assert_eq!(fitness, 72);
    }

    #[test]
    fn test_perfect_smoothness2() {
        // A board with many adjacent tiles of the same value
        let board = Board {
            board: [2, 2, 3, 3, 2, 2, 3, 3, 4, 4, 5, 5, 4, 4, 5, 5],
            score: 0,
            seed: 0,
        };
        let evaluator = SmoothnessEvaluator {};

        let fitness = evaluator.get_fitness(&board);
        assert_eq!(fitness, 52); // 12 horizontal matches + 12 vertical matches
    }

    #[test]
    fn test_good_smoothness() {
        // A board with decent smoothness (many close values)
        let board = Board {
            board: [1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 4, 4, 3, 4, 5, 5],
            score: 0,
            seed: 0,
        };
        let evaluator = SmoothnessEvaluator {};

        let fitness = evaluator.get_fitness(&board);
        assert_eq!(fitness, 38); // Mix of exact matches and close values
    }

    #[test]
    fn test_poor_smoothness() {
        // A board with poor smoothness (big jumps between values)
        let board = Board {
            board: [2, 5, 8, 11, 3, 7, 10, 13, 4, 6, 9, 12, 1, 5, 8, 11],
            score: 0,
            seed: 0,
        };
        let evaluator = SmoothnessEvaluator {};

        let fitness = evaluator.get_fitness(&board);
        assert_eq!(fitness, 8); // No neighbors with same or close values
    }

    #[test]
    fn test_with_zeros() {
        // A board with empty cells but good smoothness otherwise
        let board = Board {
            board: [2, 2, 0, 3, 0, 2, 2, 0, 3, 3, 3, 0, 0, 0, 4, 4],
            score: 0,
            seed: 0,
        };
        let evaluator = SmoothnessEvaluator {};

        let fitness = evaluator.get_fitness(&board);
        assert_eq!(fitness, 21); // Should count only non-empty adjacent cells
    }
}
