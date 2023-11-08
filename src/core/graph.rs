pub use super::edge::Edge;
pub use super::node::Node;
use super::edgeinformation::EdgeInformation;
use super::routing::RoutingResult;
use super::routingoptions::AlgorithmOptions;
pub use super::weight::WeightCalculator;
use std::collections::HashSet;

use std::rc::Rc;

pub trait Graph {
    //mut functions:
    fn add_node(&mut self, node: Node);
    fn add_edge(&mut self, edge: Edge);

    //simple non mut functions
    fn get_directed_edge_between(&self, start: i32, end: i32) -> Option<Rc<EdgeInformation>>;
    fn get_nr_nodes(&self) -> usize;
    fn get_nr_edges(&self) -> usize;

    //more complex functions
    fn do_for_all_neighbors<F>(&self, base_node: i32, reverse: bool, f: F)
    where
        F: FnMut(i32, &Rc<Edge>); //TODO this needs to become independent of actual edge storage

    fn route(&self, opts: &AlgorithmOptions<Self>, start: i32, end: i32) -> Option<RoutingResult>
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