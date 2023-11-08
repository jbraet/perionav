pub use super::edge::Edge;
pub use super::node::Node;
use super::graph::Graph;
use super::edgeinformation::EdgeInformation;
use super::routing::RoutingResult;
use super::routingoptions::AlgorithmOptions;
pub use super::weight::WeightCalculator;

use std::collections::HashMap;
use std::rc::Rc;
use std::vec;
use std::fmt;

pub struct StandardGraph {
    nodes: Vec<Node>,
    neighbors: HashMap<i32, Vec<Rc<Edge>>>, //node index to coinciding edges
    reverse_neighbors: HashMap<i32,Vec<Rc<Edge>>>,  //probably not the most efficient implementation, but create a graph2 and benchmark
}

impl Default for StandardGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl StandardGraph {
    pub fn new() -> Self {
        StandardGraph {
            nodes: vec![],
            neighbors: HashMap::new(),
            reverse_neighbors: HashMap::new(),
        }
    }
}

impl Graph for StandardGraph {
    fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, edge: Edge) {
        let edge_clone = Rc::new(edge);
        edge_clone.apply_nodes(|base_node, adj_node| {
            let neighbors_for_node = self.neighbors.entry(base_node).or_insert(vec![]);
            let reverse_neighbors_for_node = self.reverse_neighbors.entry(adj_node).or_insert(vec![]);
            
            neighbors_for_node.push(Rc::clone(&edge_clone));
            reverse_neighbors_for_node.push(Rc::clone(&edge_clone));
        });
    }

    fn do_for_all_neighbors<F>(&self, base_node: i32, reverse: bool, mut f: F)
    where
        F: FnMut(i32, &Rc<Edge>),
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

        for edge in neighbors {
            let adj_node = edge.get_adj_node(base_node);
            f(adj_node, edge);
        }
    }

    fn get_directed_edge_between(&self, start: i32, end: i32) -> Option<Rc<EdgeInformation>> {
        let edge_option = self.neighbors.get(&start);
        
        edge_option.map(|start_neighbors| {
            let result_list = start_neighbors.iter().filter(|e| e.get_adj_node(start) == end).collect::<Vec<_>>();
            if !result_list.is_empty() {
                let edge = result_list[0];
            
                Rc::new(EdgeInformation::new(Rc::clone(edge),end))
            } else {
                panic!("edge between {} and {} doesn't exist",start,end)
            }           
        })
    }

    fn get_nr_nodes(&self) -> usize {
        self.nodes.len()
    }

    fn get_nr_edges(&self) -> usize {
        self.neighbors.values().fold(0,|acc, e| acc + e.len())
    }

    fn route(&self, opts: &AlgorithmOptions<StandardGraph>, start: i32, end: i32) -> Option<RoutingResult> {
        opts.routing_algorithm.route(self, start, end)
    }
}

impl fmt::Debug for StandardGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (start_node, edges) in &self.neighbors {
            for edge in edges {
                let end_node = edge.get_adj_node(*start_node);

                writeln!(f, "{} -> {}", start_node, end_node)?;
            }
        }

        write!(f, "")
    }
}
