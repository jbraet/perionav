use std::collections::{HashMap, HashSet};

use crate::core::Graph;

use super::options::ComponentsAlgorithm;

//implementation of https://en.wikipedia.org/wiki/Path-based_strong_component_algorithm

pub struct PathBasedComponentsAlgorithm {}

struct AlgorithmData {
    current_components_stack: Vec<usize>, // S in the description; all vertexes in the preorder order. This is used to construct the final components
    cycles_stack: Vec<usize>,             // P in the description; a stack that tries to detect a strongly connected component
    //if P finds a cycle it gets popped off, however the relevant vertexes still stay on S. If P ends on the node it started then we have an actual strongly subcomponent that couldn't have been bigger, so we add it to the result
    preorder_number: usize,
    preorder_numbers: HashMap<usize, usize>, //preorder number
    is_in_component: HashSet<usize>,         //if a vertex is in the component
    components: Vec<HashSet<usize>>,         //actual return value
}

impl AlgorithmData {
    fn new() -> Self {
        AlgorithmData {
            current_components_stack: vec![],
            cycles_stack: vec![],
            preorder_number: 0,
            preorder_numbers: HashMap::new(),
            is_in_component: HashSet::new(),
            components: vec![],
        }
    }
}

impl Default for PathBasedComponentsAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}

impl PathBasedComponentsAlgorithm {
    pub fn new() -> Self {
        PathBasedComponentsAlgorithm {}
    }

    fn determine_components_from_node(&self, algorithm_data: &mut AlgorithmData, graph: &impl Graph, start_index: usize) {
        let mut stack = Vec::new();
        stack.push((start_index, true)); //boolean is whether or not we should visit the neighbors
                                         //after visiting a node, the same node will be pushed with false, so that it can be handled after all of the subtree of the current node is handled

        while let Some((current_node, visit_neighbors)) = stack.pop() {
            if visit_neighbors && !algorithm_data.preorder_numbers.contains_key(&current_node) {
                algorithm_data.preorder_numbers.insert(current_node, algorithm_data.preorder_number);
                algorithm_data.preorder_number += 1;
                algorithm_data.current_components_stack.push(current_node);
                algorithm_data.cycles_stack.push(current_node);

                stack.push((current_node, false)); // we will come back to this once all others are explored

                graph.do_for_all_neighbors(current_node, false, |adj_node| {
                    if let Some(preorder_number_adj) = algorithm_data.preorder_numbers.get(&adj_node) {
                        if !algorithm_data.is_in_component.contains(&adj_node) {
                            while algorithm_data
                                .cycles_stack
                                .last()
                                .is_some_and(|x| algorithm_data.preorder_numbers.get(x).is_some_and(|c| c > preorder_number_adj))
                            {
                                algorithm_data.cycles_stack.pop();
                            }
                        }
                    } else {
                        stack.push((adj_node, true));
                    }
                });
            } else if algorithm_data.cycles_stack.last().is_some_and(|x| *x == current_node) {
                let mut component = HashSet::new();
                while let Some(vertex) = algorithm_data.current_components_stack.pop() {
                    component.insert(vertex);
                    algorithm_data.is_in_component.insert(vertex);
                    if vertex == current_node {
                        break;
                    }
                }

                algorithm_data.components.push(component);
                algorithm_data.cycles_stack.pop();
            }
        }
    }
}

impl<G: Graph> ComponentsAlgorithm<G> for PathBasedComponentsAlgorithm {
    fn get_components(&self, graph: &G) -> Vec<HashSet<usize>> {
        let mut algorithm_data = AlgorithmData::new();

        for i in 0..graph.get_nr_nodes() {
            if !algorithm_data.is_in_component.contains(&i) {
                self.determine_components_from_node(&mut algorithm_data, graph, i)
            }
        }

        algorithm_data.components
    }
}
