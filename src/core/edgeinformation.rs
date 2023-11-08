use super::edge::DirectedVehicleSpecificEdgeInformation;
use std::rc::Rc;
use std::fmt;

//represents a directed version of an edge. By using the adj node we can easily get a direction of the edge
pub struct EdgeInformation {
    base_node: i32,
    adj_node: i32,
    
    //TODO can we remove these ? 
    from_node_lat:f64, 
    from_node_lon:f64, 
    to_node_lat:f64, 
    to_node_lon:f64,

    _edge_info: Rc<DirectedVehicleSpecificEdgeInformation>,
}

impl EdgeInformation{
    pub fn new(base_node: i32, adj_node: i32, from_node_lat:f64, from_node_lon:f64, to_node_lat:f64, to_node_lon:f64, edge_info: Rc<DirectedVehicleSpecificEdgeInformation>) -> Self {
        EdgeInformation { 
            base_node, 
            adj_node,
            from_node_lat, 
            from_node_lon, 
            to_node_lat, 
            to_node_lon,
            _edge_info: edge_info,
        }
    }

    pub fn get_base_node(&self) -> i32 {
        self.base_node
    }

    pub fn get_adj_node(&self) -> i32 {
        self.adj_node
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
        write!(f, "{} <-> {}", self.base_node, self.adj_node)
    }
}
