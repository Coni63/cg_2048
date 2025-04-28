use fxhash::{FxHashSet, FxHasher};
use std::hash::{Hash, Hasher};

use crate::agent::agent_trait::Agent;
use crate::agent::node::Node;
use crate::board::Board;
use crate::evaluator::evaluator_trait::Evaluator;
use crate::evaluator::PriorityEvaluator;

pub struct BeamSearch<T: Evaluator> {
    pub evaluator: T,
}

impl<T: Evaluator> Agent for BeamSearch<T> {
    fn search(&self, node: &mut Node) -> Node {
        self._search(node, 132000)
    }

    fn search_with_depth(&self, node: &mut Node, max_depth: i32) -> Node {
        self._search(node, max_depth)
    }
}

impl<T: Evaluator> BeamSearch<T> {
    pub fn get_child(&self, node: &Node, action: u8) -> Option<Node> {
        let mut new_board = node.board.clone();
        if new_board.play(action) {
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
        } else {
            None
        }
    }

    fn _search(&self, root: &mut Node, max_depth: i32) -> Node {
        // let mut queue_a: BinaryHeap<Node> = BinaryHeap::new();
        // let mut queue_b: BinaryHeap<Node> = BinaryHeap::new();
        let mut queue_a: Vec<Node> = Vec::new();
        let mut queue_b: Vec<Node> = Vec::new();
        let mut hashset: FxHashSet<u64> = FxHashSet::default();

        let fitness = self.evaluator.get_fitness(&root.board);
        let max_fitness = self.evaluator.get_highest_possible_fitness(&root.board);
        root.fitness = fitness as f64 / max_fitness as f64;

        queue_a.push(root.clone());

        let mut best_node = root.clone();

        let mut move_count = 0;

        while !queue_a.is_empty() {
            for node in queue_a {
                let mut has_moved = false;
                for i in 2..5 {
                    // 2, 3, 4 are the actions for left, down, right
                    if let Some(mut new_node) = self.get_child(&node, i) {
                        let mut hasher = FxHasher::default();
                        new_node.hash(&mut hasher);
                        let h1 = hasher.finish();
                        if !hashset.contains(&h1) {
                            let fitness = self.evaluator.get_fitness(&new_node.board);
                            let max_fitness =
                                self.evaluator.get_highest_possible_fitness(&new_node.board);

                            new_node.fitness = fitness as f64 / max_fitness as f64;
                            hashset.insert(h1);
                            queue_b.push(new_node);
                            has_moved = true;
                        }
                    }
                }
                if !has_moved {
                    // up is only tested if no other moves are possible
                    match self.get_child(&node, 1) {
                        Some(mut new_node) => {
                            let mut hasher = FxHasher::default();
                            new_node.hash(&mut hasher);
                            let h1 = hasher.finish();
                            if !hashset.contains(&h1) {
                                let fitness = self.evaluator.get_fitness(&new_node.board);
                                let max_fitness =
                                    self.evaluator.get_highest_possible_fitness(&new_node.board);

                                new_node.fitness = fitness as f64 / max_fitness as f64;
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

            queue_b.sort_by(|a, b| {
                b.fitness
                    .partial_cmp(&a.fitness)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            queue_b.truncate(200);
            queue_a = queue_b;
            queue_b = Vec::new();
            hashset.clear();

            move_count += 1;
            if move_count == max_depth {
                best_node = queue_a.pop().unwrap().clone();
                break;
            }
        }

        best_node
    }
}
