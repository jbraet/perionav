use std::collections::HashSet;

use crate::core::Graph;

use super::options::ComponentsAlgorithm;

pub struct KosarajuComponentsAlgorithm {}

//we create 2 algorithm data structs, because we need a reference to the first while doing a mut function on the second.
//which is impossible if this is in one struct, but it does work when its split up
struct AlgorithmData {
    visited: HashSet<usize>,
    nodes_list: Vec<usize>,
}

struct AlgorithmDataPart2 {
    components: Vec<HashSet<usize>>, //actual return value
    is_in_component: HashSet<usize>, //if a vertex is in the component
}

impl AlgorithmDataPart2 {
    fn new() -> Self {
        AlgorithmDataPart2 {
            is_in_component: HashSet::new(),
            components: vec![],
        }
    }

    fn create_component(&mut self, graph: &impl Graph, start_node: usize) {
        let mut component = HashSet::new();

        let mut stack = Vec::new();
        stack.push(start_node);

        while let Some(current_node) = stack.pop() {
            component.insert(current_node);
            self.is_in_component.insert(current_node);
            graph.do_for_all_neighbors(current_node, true, |adj_node| {
                if !component.contains(&adj_node) && !self.is_in_component.contains(&adj_node) {
                    stack.push(adj_node);
                }
            });
        }

        self.components.push(component);
    }
}

impl AlgorithmData {
    fn new() -> Self {
        AlgorithmData {
            visited: HashSet::new(),
            nodes_list: vec![],
        }
    }

    fn visit(&mut self, graph: &impl Graph, start_node: usize) {
        let mut stack = Vec::new();
        stack.push((start_node, true)); //boolean is whether or not we should visit the neighbors

        while let Some((current_node, visit_neighbors)) = stack.pop() {
            if visit_neighbors && !self.visited.contains(&current_node) {
                self.visited.insert(current_node);

                stack.push((current_node, false)); // we will come back to this once all others are explored

                graph.do_for_all_neighbors(current_node, false, |adj_node| {
                    if !self.visited.contains(&adj_node) {
                        stack.push((adj_node, true));
                    }
                });
            } else if !visit_neighbors {
                self.nodes_list.push(current_node);
            }
        }
    }
}

impl<G: Graph> ComponentsAlgorithm<G> for KosarajuComponentsAlgorithm {
    fn get_components(&self, graph: &G) -> Vec<HashSet<usize>> {
        let mut algorithm_data = AlgorithmData::new();

        for i in 0..graph.get_nr_nodes() {
            if !algorithm_data.visited.contains(&i) {
                algorithm_data.visit(graph, i);
            }
        }

        //we create a new algorithmData
        let mut algorithm_data2 = AlgorithmDataPart2::new();

        for start_node in algorithm_data.nodes_list.iter().rev() {
            if !algorithm_data2.is_in_component.contains(start_node) {
                algorithm_data2.create_component(graph, *start_node);
            }
        }

        algorithm_data2.components
    }
}
