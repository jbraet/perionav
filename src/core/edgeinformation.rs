use super::edge::Edge;
use std::rc::Rc;
use std::fmt;

//contains a directed version of an edge. By using the adj node we can easily get a direction of the edge
pub struct EdgeInformation { //TODO does this have to contain edge ?
    pub edge: Rc<Edge>,
    pub adj_node: i32,
}

impl EdgeInformation{
    pub fn new(edge: Rc<Edge>, adj_node: i32) -> Self {
        if !edge.has_node(adj_node) {
            panic!("creating edge with a node {} thats not part of the edge", adj_node)
        }

        EdgeInformation { 
            edge, 
            adj_node,
        }
    }

    pub fn get_base_node(&self) -> i32 {
        self.edge.get_adj_node(self.adj_node)
    }
}

impl fmt::Debug for EdgeInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <-> {}", self.edge.get_adj_node(self.adj_node), self.adj_node)
    }
}
