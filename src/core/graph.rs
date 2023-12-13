use kdtree::KdTree;

use super::components::options::ComponentsAlgorithmOptions;
use super::edge::DirectedVehicleSpecificEdgeInformation;
pub use super::edge::Edge;
pub use super::node::Node;
use super::routing::options::RoutingAlgorithmOptions;
use super::routing::RoutingResult;
pub use super::weight::WeightCalculator;

use std::collections::HashSet;

use std::rc::Rc;

pub trait Graph: std::fmt::Debug {
    //mut functions:
    fn add_node(&mut self, node: Node);
    fn add_edge(&mut self, base_node: i32, adj_node: i32, edge: Edge);
    fn keep_nodes(&mut self, nodes: &HashSet<i32>);

    //simple non mut functions
    fn get_directed_vehicle_specific_edge_information(
        &self,
        base_node: i32,
        adj_node: i32,
        reverse: bool,
    ) -> Option<Rc<DirectedVehicleSpecificEdgeInformation>>;
    fn get_node(&self, id: i32) -> Option<&Node>;

    fn get_nr_nodes(&self) -> usize;
    fn get_nr_edges(&self) -> usize;

    //more complex functions
    fn do_for_all_neighbors<F>(&self, base_node: i32, reverse: bool, f: F)
    where
        F: FnMut(i32);

    fn route(&self, opts: &RoutingAlgorithmOptions<Self>, start: i32, end: i32) -> Option<RoutingResult>
    where
        Self: Sized;

    //a vector of sets of nodeids, each set is a strongly connected subgraph
    fn get_strongly_connected_subgraphs(&self, opts: &ComponentsAlgorithmOptions<Self>) -> Vec<HashSet<i32>>
    where
        Self: Sized;

    //functions with default implementations
    //used for debugging certain parts of a graph
    fn visualise_sub_graph(&self, nodes: &HashSet<i32>) -> String {
        let mut used = HashSet::new();
        let mut res = vec![];
        for node in nodes {
            let node = *node;
            self.do_for_all_neighbors(node, false, |adj_node| {
                //also check if the reverse has been added already
                //its also possible that there are multiple edges between two nodes so also check the normal order
                if nodes.contains(&adj_node) && !used.contains(&(adj_node, node)) && !used.contains(&(node, adj_node)) {
                    let from = self.get_node(node).unwrap();
                    let to = self.get_node(adj_node).unwrap();

                    res.push(format!("({:.6} {:.6}, {:.6} {:.6})", from.lon, from.lat, to.lon, to.lat));
                    used.insert((node, adj_node));
                }
            })
        }

        format!("MULTILINESTRING({})", res.join(","))
    }

    fn create_kd_tree(&self) -> KdTree<f64, i32, [f64; 2]> {
        //last type param are the coordinates, second is the extra data stored and the first is just to clarify the third or something
        let mut kdtree = KdTree::new(2);

        for i in 0..self.get_nr_nodes() {
            let index = i as i32;
            let node = self.get_node(index).unwrap();
            kdtree.add([node.lat, node.lon], index).unwrap();
        }
        kdtree
    }
}
