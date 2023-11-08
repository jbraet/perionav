use super::components::options::ComponentsAlgorithmOptions;
use super::edge::DirectedVehicleSpecificEdgeInformation;
pub use super::edge::Edge;
pub use super::node::Node;
use super::graph::Graph;
use super::edgeinformation::EdgeInformation;
use super::routing::RoutingResult;
use super::routing::options::RoutingAlgorithmOptions;
pub use super::weight::WeightCalculator;

use kdtree::KdTree;
use kdtree::distance::squared_euclidean;

use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::vec;
use std::fmt;

pub struct StandardGraph {
    nodes: Vec<Node>,
    //TODO kdtree probably doesnt belong inside the graph ? 
    kdtree: KdTree<f64,i32,[f64;2]>, //last type param are the coordinates, second is the extra data stored and the first is just to clarify the third or something
    neighbors: HashMap<i32, HashMap<i32, Rc<Edge>>>, //node index to coinciding edges
    reverse_neighbors: HashMap<i32, HashMap<i32, Rc<Edge>>>,  //probably not the most efficient implementation, but create a graph2 and benchmark
}

impl Default for StandardGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl StandardGraph {
    pub fn new() -> Self {
        StandardGraph {
            kdtree: KdTree::new(2),
            nodes: vec![],
            neighbors: HashMap::new(),
            reverse_neighbors: HashMap::new(),
        }
    }
}

impl Graph for StandardGraph {
    fn add_node(&mut self, node: Node) {
        self.kdtree.add([node.lat, node.lon], self.nodes.len() as i32).unwrap();
        self.nodes.push(node);
    }

    fn add_edge(&mut self, base_node:i32, adj_node: i32,  edge: Edge) {
        let fwd = edge.is_forward(super::edge::VehicleTypes::Car);
        let bwd = edge.is_backward(super::edge::VehicleTypes::Car);

        let reverse_edge = Rc::new(edge.create_opposite());
        let rc_edge = Rc::new(edge);

        if fwd {
            let neighbors_for_node = self.neighbors.entry(base_node).or_default();
            let reverse_neighbors_for_node = self.reverse_neighbors.entry(adj_node).or_default();
            neighbors_for_node.insert(adj_node, Rc::clone(&rc_edge));
            reverse_neighbors_for_node.insert(base_node, Rc::clone(&reverse_edge));
        }
        
        if bwd {
            let neighbors_for_node = self.neighbors.entry(adj_node).or_default();
            let reverse_neighbors_for_node = self.reverse_neighbors.entry(base_node).or_default();
            neighbors_for_node.insert(base_node, Rc::clone(&reverse_edge));
            reverse_neighbors_for_node.insert(adj_node, Rc::clone(&rc_edge));
        }
    }

    fn keep_nodes(&mut self, nodes: &HashSet<i32>) { 
        let mut node_map = HashMap::new();
        let mut node_counter = 0;
        let mut new_nodes = vec![];
        for i in 0..self.nodes.len() {
            if nodes.contains(&(i as i32)) {
                node_map.insert(i, node_counter);
                node_counter+=1;
            
                let node = self.nodes.get(i).copied().unwrap_or_default();
                new_nodes.push(node);
            }

            let index = i as i32;
            let mut remove_neighbors = vec![]; //neighbors of this node
            self.do_for_all_neighbors(index, false, |adj_node, _| {
                if !nodes.contains(&index) || !nodes.contains(&adj_node) {
                    remove_neighbors.push(adj_node); // we can't modify directly in here because we are also reading the neighbors in the func
                }
            });

            for adj_node in remove_neighbors {
                let remove = self.neighbors.get_mut(&index).is_some_and(|map| {map.remove(&adj_node); map.is_empty()});
                if remove {
                    self.neighbors.remove(&index);
                }
                let remove = self.reverse_neighbors.get_mut(&adj_node).is_some_and(|map| {map.remove(&adj_node); map.is_empty()});
                if remove {
                    self.reverse_neighbors.remove(&index);
                }
            }
        }

        self.nodes = new_nodes;

        //TODO do something about the kdtree
    }

    fn get_node(&self,id: i32) -> Option<&Node> {
        self.nodes.get(id as usize)
    }

    fn find_closest_node(&self, lat: f64, lon: f64) -> i32 {
        let kd_nodes = self.kdtree.nearest(&[lat, lon], 1, &squared_euclidean).unwrap();
        *kd_nodes[0].1
    }

    fn do_for_all_neighbors<F>(&self, base_node: i32, reverse: bool, mut f: F)
    where
        F: FnMut(i32, Rc<DirectedVehicleSpecificEdgeInformation>),
    {
        let relevant_neighbors = if reverse {
            &self.reverse_neighbors
        } else {
            &self.neighbors
        };

        let neighbors = match relevant_neighbors.get(&base_node) {
            None => return, //there are no neighbors so do nothing
            Some(n) => n,
        };

        for (adj_node, edge) in neighbors {
            let directed_edge_info = edge.get_directed_vehicle_specific_edge_information(super::edge::VehicleTypes::Car, reverse).unwrap();
            f(*adj_node, directed_edge_info);
        }
    }

    fn get_directed_edge_between(&self, start: i32, end: i32) -> Option<Rc<EdgeInformation>> {
        let neighbors_option = self.neighbors.get(&start);
        
        let edge_option = neighbors_option.and_then(|start_neighbors| start_neighbors.get(&end));
        edge_option.map(|edge| {
            let edge_info = edge.get_directed_vehicle_specific_edge_information(super::edge::VehicleTypes::Car, false).unwrap();

            let actual_start_node = self.get_node(start).unwrap();
            let actual_end_node = self.get_node(end).unwrap();
        
            Rc::new(EdgeInformation::new(start, end, actual_start_node.lat, actual_start_node.lon, actual_end_node.lat, actual_end_node.lon, edge_info))
        })
    }

    fn get_nr_nodes(&self) -> usize {
        self.nodes.len()
    }

    fn get_nr_edges(&self) -> usize {
        self.neighbors.values().fold(0,|acc, e| acc + e.len())
    }

    fn route(&self, opts: &RoutingAlgorithmOptions<StandardGraph>, start: i32, end: i32) -> Option<RoutingResult> {
        opts.routing_algorithm.route(self, start, end)
    }

    fn get_strongly_connected_subgraphs(&self, opts: &ComponentsAlgorithmOptions<StandardGraph>) -> Vec<HashSet<i32>> {
        opts.components_algorithm.get_components(self)
    }
}

impl fmt::Debug for StandardGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (start_node, edges) in &self.neighbors {
            for end_node in edges.keys() {
                writeln!(f, "{} -> {}", start_node, end_node)?;
            }
        }

        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_do_for_all_neighbors() {
        let mut graph = StandardGraph::new();
        graph.add_node(Node::default());
        graph.add_node(Node::default());
        graph.add_node(Node::default());
        graph.add_node(Node::default());
        graph.add_node(Node::default());
        graph.add_node(Node::default());

        graph.add_edge(0, 1, Edge::new(1.0, true, true));
        graph.add_edge(0, 2, Edge::new(1.0, true, true));
        graph.add_edge(0, 3, Edge::new(1.0, true, true));
        graph.add_edge(1, 2, Edge::new(1.0, true, true));
        graph.add_edge(4, 5, Edge::new(1.0, true, true));

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(2, false, |adj_node, _| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==2 && adj_nodes.contains(&0) && adj_nodes.contains(&1),"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(0, false, |adj_node, _| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==3 && adj_nodes.contains(&1) && adj_nodes.contains(&2) && adj_nodes.contains(&3),"adj nodes: {:?}",adj_nodes);
    }

     #[test]
    fn test_edge_directions() {
        let mut graph = StandardGraph::new();
        graph.add_node(Node::default());
        graph.add_node(Node::default());

        graph.add_node(Node::default());
        graph.add_node(Node::default());

        graph.add_edge(0, 1, Edge::new(1.0, true, false));
        graph.add_edge(2, 3, Edge::new(1.0, false, true));

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(0, false, |adj_node, _| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==1 && adj_nodes.contains(&1),"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(0, true, |adj_node, _| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==0,"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(1, false, |adj_node, _| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==0,"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(1, true, |adj_node, _| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==1 && adj_nodes.contains(&0),"adj nodes: {:?}",adj_nodes);

        //now test the edge in the other direction:
        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(3, false, |adj_node, _| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==1 && adj_nodes.contains(&2),"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(3, true, |adj_node, _| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==0,"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(2, false, |adj_node, _| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==0,"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(2, true, |adj_node, _| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==1 && adj_nodes.contains(&3),"adj nodes: {:?}",adj_nodes);
    }
}
