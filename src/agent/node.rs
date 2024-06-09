use std::hash::{Hash, Hasher};

use crate::board::Board;

pub struct Node {
    pub board: Board,
    pub fitness: u64,
    pub action: String,
}

impl Node {
    pub fn new(board: &Board) -> Self {
        Node {
            board: board.clone(),
            fitness: 0u64,
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
