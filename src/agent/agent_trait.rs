use crate::agent::node::Node;
use crate::board::Board;

pub trait Agent {
    fn search(&self, board: &Board) -> Node;
}
