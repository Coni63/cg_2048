use crate::agent::node::Node;
use crate::board::Board;

pub trait Agent {
    fn search(&self, board: &Board) -> Node;

    fn search_with_depth(&self, board: &Board, max_depth: i32) -> Node;
}
