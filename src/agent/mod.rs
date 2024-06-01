use fxhash::{FxHashSet, FxHasher};
use std::hash::{Hash, Hasher};

use crate::board::Board;

pub struct Node {
    pub board: Board,
    pub fitness: u64,
    pub action: String,
}

impl Node {
    fn new(board: &Board) -> Self {
        Node {
            board: board.clone(),
            fitness: get_fitness(&board.board),
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

pub fn get_fitness(grid: &[u8; 16]) -> u64 {
    // let powers: [u8; 16] = [1, 2, 4, 6, 14, 12, 10, 8, 16, 18, 20, 22, 30, 28, 26, 24];
    let powers: [u8; 16] = [0, 0, 0, 0, 8, 6, 4, 2, 10, 12, 14, 16, 24, 22, 20, 18];

    grid.iter().zip(&powers).fold(0, |acc, (&x, &y)| {
        acc + if (x > 0) && (y > 0) { 1 << (x + y) } else { 0 }
    })
}

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

pub fn beam_search(board: &Board) -> Node {
    let mut queue_a: Vec<Node> = Vec::new();
    let mut queue_b: Vec<Node> = Vec::new();
    let mut hashset: FxHashSet<u64> = FxHashSet::default();

    let root = Node::new(board);
    queue_a.push(root.clone());

    let mut best_node = root.clone();

    while !queue_a.is_empty() {
        for node in queue_a {
            let mut has_moved = false;
            for i in 2..5 {
                if let Some(new_node) = get_child(&node, i) {
                    let mut hasher = FxHasher::default();
                    new_node.hash(&mut hasher);
                    let h1 = hasher.finish();
                    if !hashset.contains(&h1) {
                        hashset.insert(h1);
                        queue_b.push(new_node);
                        has_moved = true;
                    }
                }
            }
            if !has_moved {
                match get_child(&node, 1) {
                    Some(new_node) => {
                        let mut hasher = FxHasher::default();
                        new_node.hash(&mut hasher);
                        let h1 = hasher.finish();
                        if !hashset.contains(&h1) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fitness() {
        let board = [0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 5, 4, 3, 2];
        let expected = (1 << 25) + (1 << 22) + (1 << 19) + (1 << 16) + (1 << 3);
        let fitness = get_fitness(&board);
        assert_eq!(fitness, expected);
    }
}
