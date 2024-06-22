use crate::agent::node::Node;

pub trait Agent {
    fn search(&self, node: &mut Node) -> Node;

    fn search_with_depth(&self, node: &mut Node, max_depth: i32) -> Node;
}
