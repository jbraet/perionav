use super::components::options::ComponentsAlgorithmOptions;
use super::edge::DirectedVehicleSpecificEdgeInformation;
pub use super::edge::Edge;
pub use super::node::Node;
use super::graph::Graph;
use super::routing::RoutingResult;
use super::routing::options::RoutingAlgorithmOptions;
pub use super::weight::WeightCalculator;

use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::fmt;

//since we will assume the graph is (nearly (before the filtering)) strongly connected we can assume that each node will have at least one edge to another node
pub struct StandardGraph {
    nodes: Vec<Node>,
    neighbors: Vec<HashMap<i32, Rc<Edge>>>, //node index to coinciding edges
    reverse_neighbors: Vec<HashMap<i32, Rc<Edge>>>,  //probably not the most efficient implementation, but create a graph2 and benchmark
}

impl Default for StandardGraph {
    fn default() -> Self {
        Self::new(100)
    }
}

impl StandardGraph {
    // nr_nodes is a suggestion to avoid resizing too much
    pub fn new(nr_nodes: usize) -> Self {
        StandardGraph {
            nodes: Vec::with_capacity(nr_nodes),
            neighbors: Vec::with_capacity(nr_nodes),
            reverse_neighbors: Vec::with_capacity(nr_nodes),
        }
    }
}

impl Graph for StandardGraph {
    fn add_node(&mut self, node: Node) {
        self.nodes.push(node);

        //nodes and neighbors start at the same size, so if its greater it can only be because of this call so only need to increase the size by one
        if self.nodes.len()>self.neighbors.len() {
            self.neighbors.push(Default::default());
            self.reverse_neighbors.push(Default::default());
        }
    }

    fn add_edge(&mut self, base_node:i32, adj_node: i32,  edge: Edge) {
        assert!((base_node as usize)<self.nodes.len() && (adj_node as usize)<self.nodes.len());

        let fwd = edge.is_forward(super::edge::VehicleTypes::Car);
        let bwd = edge.is_backward(super::edge::VehicleTypes::Car);

        let reverse_edge = Rc::new(edge.create_opposite());
        let rc_edge = Rc::new(edge);

        if fwd {
            self.neighbors[base_node as usize].insert(adj_node, Rc::clone(&rc_edge));
            self.reverse_neighbors[adj_node as usize].insert(base_node, Rc::clone(&reverse_edge));
        }
        
        if bwd {
            self.neighbors[adj_node as usize].insert(base_node, Rc::clone(&reverse_edge));
            self.reverse_neighbors[base_node as usize].insert(adj_node, Rc::clone(&rc_edge));
        }
    }

    fn keep_nodes(&mut self, nodes: &HashSet<i32>) {
        let mut index = 0;
        let mut remaining_index=0_i32;
        let mut nodes_map = HashMap::new(); //key>=value 

        self.nodes.retain(|_| {
            let ret = nodes.contains(&index);
            
            if ret {
                nodes_map.insert(index, remaining_index);
                remaining_index+=1;
            }

            index+=1;
            ret
        });

        let mut index = -1;
        self.neighbors.retain(|_| {
            index+=1;
            nodes.contains(&index)
        });

        let mut index = -1;
        self.reverse_neighbors.retain(|_| {
            index+=1;
            nodes.contains(&index)
        });

        for i in 0..self.nodes.len() {
            let adj_nodes = match self.neighbors.get(i) {
                None => continue, //there are no neighbors so do nothing
                Some(n) => n,
            };

            let mut new_adj_nodes = HashMap::new();
            for (k,v) in adj_nodes.iter() {
                if nodes_map.contains_key(k) {
                    new_adj_nodes.insert(*nodes_map.get(k).unwrap(),Rc::clone(v));
                }
            }

            self.neighbors[i]=new_adj_nodes;
        }

        for i in 0..self.nodes.len() { //separate loops because otherwise the continue doesn't work
            let reverse_adj_nodes = match self.reverse_neighbors.get(i) {
                None => continue, //there are no neighbors so do nothing
                Some(n) => n,
            };

            let mut new_reverse_adj_nodes = HashMap::new();
            for (k,v) in reverse_adj_nodes.iter() {
                if nodes_map.contains_key(k) {
                    new_reverse_adj_nodes.insert(*nodes_map.get(k).unwrap(),Rc::clone(v));
                }
            }

            self.reverse_neighbors[i] = new_reverse_adj_nodes;
        }
    }



    fn get_node(&self,id: i32) -> Option<&Node> {
        self.nodes.get(id as usize)
    }

    fn do_for_all_neighbors<F>(&self, base_node: i32, reverse: bool, mut f: F)
    where
        F: FnMut(i32),
    {
        let relevant_neighbors = if reverse {
            &self.reverse_neighbors
        } else {
            &self.neighbors
        };

        let neighbors = match relevant_neighbors.get(base_node as usize) {
            None => return, //there are no neighbors so do nothing
            Some(n) => n,
        };

        for adj_node in neighbors.keys() {
            f(*adj_node);
        }
    }

    fn get_directed_vehicle_specific_edge_information(&self, base_node: i32, adj_node: i32, reverse: bool) ->  Option<Rc<DirectedVehicleSpecificEdgeInformation>> {
        let relevant_neighbors = if reverse {
            &self.reverse_neighbors
        } else {
            &self.neighbors
        };

        relevant_neighbors.get(base_node as usize).and_then(|n| {
            n.get(&adj_node).and_then(|e| {
                e.get_directed_vehicle_specific_edge_information(super::edge::VehicleTypes::Car, reverse)
            })
        })
    }

    fn get_nr_nodes(&self) -> usize {
        self.nodes.len()
    }

    fn get_nr_edges(&self) -> usize {
        self.neighbors.iter().fold(0,|acc, e| acc + e.len())
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
        for (start_node, edges) in self.neighbors.iter().enumerate() {
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
    fn test_keep_nodes() {
        let mut g = StandardGraph::new(100);
        g.add_node(Node::default());
        g.add_node(Node::default());
        g.add_node(Node::default());
        g.add_node(Node::default());
        g.add_node(Node::default());
        g.add_node(Node::default());
        g.add_node(Node::default());
        g.add_node(Node::default());

        g.add_edge(0, 1, Edge::new(1.0, true, false));
        g.add_edge(1, 2, Edge::new(1.0, true, false));
        g.add_edge(2, 0, Edge::new(1.0, true, false));

        g.add_edge(3, 1, Edge::new(1.0, true, false));
        g.add_edge(3, 2, Edge::new(1.0, true, false));
        g.add_edge(3, 4, Edge::new(1.0, true, true));

        g.add_edge(4, 5, Edge::new(1.0, true, false));
        g.add_edge(5, 2, Edge::new(1.0, true, false));
        g.add_edge(5, 6, Edge::new(1.0, true, true));

        g.add_edge(7, 4, Edge::new(1.0, true, false));
        g.add_edge(7, 6, Edge::new(1.0, true, false));

        let mut nodes = HashSet::new();
        nodes.insert(0);
        nodes.insert(1);
        nodes.insert(2);

        g.keep_nodes(&nodes);

        assert_eq!(g.get_nr_nodes(),3);

        let mut adj_nodes=HashSet::new();
        g.do_for_all_neighbors(0, false, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==1 && adj_nodes.contains(&1),"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        g.do_for_all_neighbors(1, false, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==1 && adj_nodes.contains(&2),"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        g.do_for_all_neighbors(2, false, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==1 && adj_nodes.contains(&0),"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        g.do_for_all_neighbors(0, true, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==1 && adj_nodes.contains(&2),"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        g.do_for_all_neighbors(1, true, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==1 && adj_nodes.contains(&0),"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        g.do_for_all_neighbors(2, true, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==1 && adj_nodes.contains(&1),"adj nodes: {:?}",adj_nodes);
    }

    #[test]
    fn test_do_for_all_neighbors() {
        let mut graph = StandardGraph::new(100);
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
        graph.do_for_all_neighbors(2, false, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==2 && adj_nodes.contains(&0) && adj_nodes.contains(&1),"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(0, false, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==3 && adj_nodes.contains(&1) && adj_nodes.contains(&2) && adj_nodes.contains(&3),"adj nodes: {:?}",adj_nodes);
    }

     #[test]
    fn test_edge_directions() {
        let mut graph = StandardGraph::new(100);
        graph.add_node(Node::default());
        graph.add_node(Node::default());

        graph.add_node(Node::default());
        graph.add_node(Node::default());

        graph.add_edge(0, 1, Edge::new(1.0, true, false));
        graph.add_edge(2, 3, Edge::new(1.0, false, true));

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(0, false, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==1 && adj_nodes.contains(&1),"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(0, true, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==0,"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(1, false, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==0,"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(1, true, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==1 && adj_nodes.contains(&0),"adj nodes: {:?}",adj_nodes);

        //now test the edge in the other direction:
        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(3, false, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==1 && adj_nodes.contains(&2),"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(3, true, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==0,"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(2, false, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==0,"adj nodes: {:?}",adj_nodes);

        let mut adj_nodes=HashSet::new();
        graph.do_for_all_neighbors(2, true, |adj_node| {
            adj_nodes.insert(adj_node);
        });
        assert!(adj_nodes.len()==1 && adj_nodes.contains(&3),"adj nodes: {:?}",adj_nodes);
    }
}
