use fxhash::{FxHashSet, FxHasher};
use std::hash::{Hash, Hasher};

use crate::agent::agent_trait::Agent;
use crate::agent::node::Node;
use crate::board::Board;
use crate::evaluator::basic_evaluator::BasicEvaluator;
use crate::evaluator::evaluator_trait::Evaluator;

pub struct BeamSearch {
    evaluator: BasicEvaluator,
}

impl BeamSearch {
    pub fn get_child(&self, node: &Node, action: u8) -> Option<Node> {
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
    fn new(evaluator: BasicEvaluator) -> Self {
        BeamSearch { evaluator }
    }

    fn search(&self, board: &Board) -> Node {
        let mut queue_a: Vec<Node> = Vec::new();
        let mut queue_b: Vec<Node> = Vec::new();
        let mut hashset: FxHashSet<u64> = FxHashSet::default();

        let mut root = Node::new(board);
        root.fitness = self.evaluator.get_fitness(&root.board);
        queue_a.push(root.clone());

        let mut best_node = root.clone();

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
                            new_node.fitness = self.evaluator.get_fitness(&new_node.board);
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
                                new_node.fitness = self.evaluator.get_fitness(&new_node.board);
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
