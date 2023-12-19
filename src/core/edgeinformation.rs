use super::edge::DirectedVehicleSpecificEdgeInformation;
use std::fmt;
use std::rc::Rc;

//represents a directed version of an edge. By using the adj node we can easily get a direction of the edge
pub struct EdgeInformation {
    base_node: usize,
    adj_node: usize,

    _edge_info: Rc<DirectedVehicleSpecificEdgeInformation>,
}

impl EdgeInformation {
    pub fn new(base_node: usize, adj_node: usize, edge_info: Rc<DirectedVehicleSpecificEdgeInformation>) -> Self {
        EdgeInformation {
            base_node,
            adj_node,
            _edge_info: edge_info,
        }
    }

    pub fn get_base_node(&self) -> usize {
        self.base_node
    }

    pub fn get_adj_node(&self) -> usize {
        self.adj_node
    }
}

impl fmt::Debug for EdgeInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <-> {}", self.base_node, self.adj_node)
    }
}
