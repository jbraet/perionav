pub use super::edge::Edge;
pub use super::node::Node;
use super::edgeinformation::EdgeInformation;
use super::routing::RoutingResult;
use super::routing::options::RoutingAlgorithmOptions;
pub use super::weight::WeightCalculator;

use std::collections::HashSet;

use std::rc::Rc;

pub trait Graph {
    //mut functions:
    fn add_node(&mut self, node: Node);
    fn add_edge(&mut self, edge: Edge);

    //simple non mut functions
    fn get_directed_edge_between(&self, start: i32, end: i32) -> Option<Rc<EdgeInformation>>;
    fn get_node(&self,id: usize) -> Option<&Node>;
    fn get_nr_nodes(&self) -> usize;
    fn get_nr_edges(&self) -> usize;

    //more complex functions
    fn find_closest_node(&self, lat: f64, lon: f64) -> i32;

    fn do_for_all_neighbors<F>(&self, base_node: i32, reverse: bool, f: F)
    where
        F: FnMut(i32, &Rc<Edge>); //TODO this needs to become independent of actual edge storage

    fn route(&self, opts: &RoutingAlgorithmOptions<Self>, start: i32, end: i32) -> Option<RoutingResult>
    where
        Self:Sized;

    //a vector of sets of nodeids, each set is a strongly connected subgraph
    fn get_strongly_connected_subgraphs(&self, opts: &RoutingAlgorithmOptions<Self>) -> Vec<HashSet<i32>> 
    where 
        Self:Sized;

    //functions with default implementations
    fn is_strongly_connected(&self) -> bool {
        let mut index = 0;

        let mut stack = vec![index];
        let mut used = HashSet::from([index]);

        while !stack.is_empty() {
            index = stack.pop().unwrap();

            self.do_for_all_neighbors(index, false, |adj_node, _| {
                if !used.contains(&adj_node) {
                    used.insert(adj_node);
                    stack.push(adj_node);
                }
            })
        }

        used.len() == self.get_nr_nodes()
    }
}