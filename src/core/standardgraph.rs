use super::components::options::ComponentsAlgorithmOptions;
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
    kdtree: KdTree<f64,i32,[f64;2]>, //last type param are the coordinates, second is the extra data stored and the first is just to clarify the third or something
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

    fn add_edge(&mut self, edge: Edge) {
        let edge_clone = Rc::new(edge);
        edge_clone.apply_nodes(|base_node, adj_node| {
            let neighbors_for_node = self.neighbors.entry(base_node).or_default();
            let reverse_neighbors_for_node = self.reverse_neighbors.entry(adj_node).or_default();
            
            neighbors_for_node.push(Rc::clone(&edge_clone));
            reverse_neighbors_for_node.push(Rc::clone(&edge_clone));
        });
    }

    fn get_node(&self,id: usize) -> Option<&Node> {
        self.nodes.get(id)
    }

    fn find_closest_node(&self, lat: f64, lon: f64) -> i32 {
        let kd_nodes = self.kdtree.nearest(&[lat, lon], 1, &squared_euclidean).unwrap();
        *kd_nodes[0].1
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

                let actual_start_node = self.get_node(start as usize).unwrap();
                let actual_end_node = self.get_node(end as usize).unwrap();
            
                Rc::new(EdgeInformation::new(Rc::clone(edge), end, actual_start_node.lat, actual_start_node.lon, actual_end_node.lat, actual_end_node.lon))
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
            for edge in edges {
                let end_node = edge.get_adj_node(*start_node);

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
    fn internal() {
        let mut graph = StandardGraph::new();
        graph.add_node(Node::default());
        graph.add_node(Node::default());
        graph.add_node(Node::default());
        graph.add_node(Node::default());
        graph.add_node(Node::default());
        graph.add_node(Node::default());

        graph.add_edge(Edge::new(0, 1, 1.0, true, true));
        graph.add_edge(Edge::new(0, 2, 1.0, true, true));
        graph.add_edge(Edge::new(0, 3, 1.0, true, true));
        graph.add_edge(Edge::new(1, 2, 1.0, true, true));
        graph.add_edge(Edge::new(4, 5, 1.0, true, true));

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
}
