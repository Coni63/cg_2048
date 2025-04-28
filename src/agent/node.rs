use std::hash::{Hash, Hasher};

use crate::board::Board;

pub struct Node {
    pub board: Board,
    pub fitness: f64,
    pub action: String,
}

impl Node {
    pub fn new(board: &Board) -> Self {
        Node {
            board: board.clone(),
            fitness: 0f64,
            action: String::new(),
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            board: self.board.clone(),
            fitness: self.fitness,
            action: self.action.clone(),
        }
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fitness
            .partial_cmp(&other.fitness)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
