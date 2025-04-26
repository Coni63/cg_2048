use fxhash::{FxHashSet, FxHasher};
use std::hash::{Hash, Hasher};

use crate::agent::agent_trait::Agent;
use crate::agent::node::Node;
use crate::board::Board;
use crate::evaluator::evaluator_trait::Evaluator;

pub struct BeamSearchV2<T: Evaluator> {
    pub evaluator: T,
}

impl<T: Evaluator> Agent for BeamSearchV2<T> {
    fn search(&self, node: &mut Node) -> Node {
        self._search(node, 132000)
    }

    fn search_with_depth(&self, node: &mut Node, max_depth: i32) -> Node {
        self._search(node, max_depth)
    }
}

impl<T: Evaluator> BeamSearchV2<T> {
    pub fn get_child(&self, node: &Node, actions: &[u8]) -> Option<Node> {
        let mut new_board = node.board.clone();
        let mut action_str = node.action.clone();

        for action in actions.iter() {
            match new_board.play(*action) {
                true => {
                    action_str += match action {
                        1 => "U",
                        2 => "L",
                        3 => "D",
                        4 => "R",
                        _ => "",
                    };
                }
                false => return None,
            }
        }

        Some(Node {
            board: new_board,
            action: action_str,
            fitness: 0,
        })
    }

    fn _search(&self, root: &mut Node, max_depth: i32) -> Node {
        // let mut queue_a: BinaryHeap<Node> = BinaryHeap::new();
        // let mut queue_b: BinaryHeap<Node> = BinaryHeap::new();
        let mut queue_a: Vec<Node> = Vec::new();
        let mut queue_b: Vec<Node> = Vec::new();
        let mut hashset: FxHashSet<u64> = FxHashSet::default();

        let actions_list: [[u8; 2]; 10] = [
            [3, 2], // DL
            [3, 4], // DR
            [3, 3], // DD
            [2, 2], // LL
            [2, 4], // LR
            [2, 3], // LD
            [4, 2], // RL
            [4, 4], // RR
            [4, 3], // RD
            [1, 3], // UD
        ];
        // 1234 ULDR

        root.fitness = self.evaluator.get_fitness(&root.board);
        queue_a.push(root.clone());

        let mut best_node = root.clone();

        let mut move_count = 0;
        while !queue_a.is_empty() {
            for node in queue_a {
                for actions in actions_list.iter() {
                    // 2, 3, 4 are the actions for left, down, right
                    if let Some(mut new_node) = self.get_child(&node, actions) {
                        let mut hasher = FxHasher::default();
                        new_node.hash(&mut hasher);
                        let h1 = hasher.finish();
                        if !hashset.contains(&h1) {
                            new_node.fitness = self.evaluator.get_fitness(&new_node.board);
                            hashset.insert(h1);
                            queue_b.push(new_node);
                        }
                    }
                }
            }

            queue_b.sort_by(|a, b| b.fitness.cmp(&a.fitness));
            queue_b.truncate(200);
            queue_a = queue_b;
            queue_b = Vec::new();
            hashset.clear();

            move_count += 2;
            if move_count == max_depth {
                best_node = queue_a.pop().unwrap().clone();
                break;
            }
        }

        best_node
    }
}
