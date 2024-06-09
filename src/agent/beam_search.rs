use fxhash::{FxHashSet, FxHasher};
use std::hash::{Hash, Hasher};

use crate::agent::agent_trait::Agent;
use crate::agent::node::Node;
use crate::board::Board;

pub struct BeamSearch {}

impl BeamSearch {
    pub fn get_child(node: &Node, action: u8) -> Option<Node> {
        let mut new_board = node.board.clone();
        match new_board.play(action) {
            true => {
                let mut new_node = Node::new(&new_board);
                new_node.action = node.action.clone()
                    + match action {
                        1 => "U",
                        2 => "L",
                        3 => "D",
                        4 => "R",
                        _ => "",
                    };
                Some(new_node)
            }
            false => None,
        }
    }
}

impl Agent for BeamSearch {
    fn get_fitness(grid: &[u8; 16]) -> u64 {
        // let powers: [u8; 16] = [1, 2, 4, 6, 14, 12, 10, 8, 16, 18, 20, 22, 30, 28, 26, 24];
        let powers: [u8; 16] = [2, 1, 0, 0, 4, 2, 1, 0, 6, 8, 10, 12, 20, 18, 16, 14];

        grid.iter().zip(&powers).fold(0, |acc, (&x, &y)| {
            acc + if x + y > 0 { 1 << (x + y) } else { 0 }
        })
    }

    fn search(board: &Board) -> Node {
        let mut queue_a: Vec<Node> = Vec::new();
        let mut queue_b: Vec<Node> = Vec::new();
        let mut hashset: FxHashSet<u64> = FxHashSet::default();

        let mut root = Node::new(board);
        root.fitness = Self::get_fitness(&root.board.board);
        queue_a.push(root.clone());

        let mut best_node = root.clone();

        while !queue_a.is_empty() {
            for node in queue_a {
                let mut has_moved = false;
                for i in 2..5 {
                    // 2, 3, 4 are the actions for left, down, right
                    if let Some(mut new_node) = Self::get_child(&node, i) {
                        let mut hasher = FxHasher::default();
                        new_node.hash(&mut hasher);
                        let h1 = hasher.finish();
                        if !hashset.contains(&h1) {
                            new_node.fitness = Self::get_fitness(&new_node.board.board);
                            hashset.insert(h1);
                            queue_b.push(new_node);
                            has_moved = true;
                        }
                    }
                }
                if !has_moved {
                    // up is only tested if no other moves are possible
                    match Self::get_child(&node, 1) {
                        Some(mut new_node) => {
                            let mut hasher = FxHasher::default();
                            new_node.hash(&mut hasher);
                            let h1 = hasher.finish();
                            if !hashset.contains(&h1) {
                                new_node.fitness = Self::get_fitness(&new_node.board.board);
                                hashset.insert(h1);
                                queue_b.push(new_node);
                            }
                        }
                        None => {
                            if node.board.score > best_node.board.score {
                                best_node = node.clone();
                            }
                        }
                    };
                }
            }

            queue_b.sort_by(|a, b| b.fitness.cmp(&a.fitness));
            queue_b.truncate(200);
            queue_a = queue_b;
            queue_b = Vec::new();
            hashset.clear();
        }

        best_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fitness() {
        let board = [0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 5, 4, 3, 2];
        let expected = (1 << 25) + (1 << 22) + (1 << 19) + (1 << 16) + (1 << 3);
        let fitness = BeamSearch::get_fitness(&board);
        assert_eq!(fitness, expected);
    }
}
