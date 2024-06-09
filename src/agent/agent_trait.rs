use crate::agent::node::Node;
use crate::board::Board;

pub trait Agent {
    fn get_fitness(grid: &[u8; 16]) -> u64;
    fn search(board: &Board) -> Node;
}
