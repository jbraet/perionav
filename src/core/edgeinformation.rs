use super::edge::Edge;
use std::rc::Rc;
use std::fmt;

//represents a directed version of an edge. By using the adj node we can easily get a direction of the edge
pub struct EdgeInformation { //TODO does this have to contain edge ?
    edge: Rc<Edge>,
    pub adj_node: i32,
    from_node_lat:f64, 
    from_node_lon:f64, 
    to_node_lat:f64, 
    to_node_lon:f64
}

impl EdgeInformation{
    pub fn new(edge: Rc<Edge>, adj_node: i32, from_node_lat:f64, from_node_lon:f64, to_node_lat:f64, to_node_lon:f64) -> Self {
        if !edge.has_node(adj_node) {
            panic!("creating edge with a node {} thats not part of the edge", adj_node)
        }

        EdgeInformation { 
            edge, 
            adj_node,
            from_node_lat, 
            from_node_lon, 
            to_node_lat, 
            to_node_lon,
        }
    }

    pub fn get_base_node(&self) -> i32 {
        self.edge.get_adj_node(self.adj_node)
    }

    pub fn get_from_coordinates(&self) -> (f64, f64) {
        (self.from_node_lat, self.from_node_lon)
    }

    pub fn get_to_coordinates(&self) -> (f64, f64) {
        (self.to_node_lat, self.to_node_lon)
    }
}

impl fmt::Debug for EdgeInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <-> {}", self.edge.get_adj_node(self.adj_node), self.adj_node)
    }
}
