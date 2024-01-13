use std::{collections::HashSet, fmt};

use crate::core::Graph;

use super::options::ComponentsAlgorithm;

/*
this iterative implementation has been translated from some java implementation
*/
pub struct TarjanComponentsAlgorithm2 {}

#[derive(Debug)]
struct AlgorithmNode {
    index: usize, // the index in which order the graph is explored
    low_link: usize,
    on_stack: bool,
}

struct AlgorithmData {
    index: usize,
    stack: Vec<usize>,

    nodes: Vec<Option<Box<AlgorithmNode>>>,
    components: Vec<HashSet<usize>>, //actual return value
}

enum State {
    BuildComponent,
    UpdateLowLinks(usize), //after visiting a single neighbor we have to do an update function based on the neighbor visited
    HandleNeighbor(usize),
    FindComponent,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::BuildComponent => write!(f, "buildComponent"),
            State::UpdateLowLinks(adj) => write!(f, "update {}", adj),
            State::HandleNeighbor(adj) => write!(f, "handleNeighbor {}", adj),
            State::FindComponent => write!(f, "findComponent"),
        }
    }
}

impl AlgorithmData {
    fn new(nr_nodes: usize) -> Self {
        let mut nodes = Vec::with_capacity(nr_nodes);
        nodes.resize_with(nr_nodes, || None);

        AlgorithmData {
            index: 0,
            stack: vec![],
            nodes,
            components: vec![],
        }
    }

    fn strongconnect_iterative(&mut self, graph: &impl Graph, node_index: usize) {
        let mut stack: Vec<(usize, State)> = Vec::new();
        stack.push((node_index, State::FindComponent));

        while let Some((current_node_index, visit_neighbors)) = stack.pop() {
            match visit_neighbors {
                State::BuildComponent => {
                    let v = self.nodes[current_node_index].as_ref().unwrap();
                    if v.low_link == v.index {
                        let mut component = HashSet::new();

                        let mut test_function = |w_index: usize| {
                            self.nodes[w_index].as_mut().unwrap().on_stack = false;

                            component.insert(w_index);
                            w_index != current_node_index
                        };
                        while self.stack.pop().is_some_and(&mut test_function) {}

                        self.components.push(component);
                    }
                }
                State::UpdateLowLinks(adj_node) => {
                    let w_low_link = self.nodes[adj_node].as_ref().unwrap().low_link;
                    let v = self.nodes[current_node_index].as_mut().unwrap();

                    if w_low_link < v.low_link {
                        v.low_link = w_low_link;
                    }
                }
                State::HandleNeighbor(adj_node) => {
                    match self.nodes[adj_node].as_ref() {
                        Some(w) => {
                            let w_index = w.index;
                            let w_on_stack = w.on_stack; //w can be dropped now

                            if w_on_stack {
                                let v = self.nodes[current_node_index].as_mut().unwrap();

                                if w_index < v.low_link {
                                    v.low_link = w_index;
                                }
                            }
                        }
                        None => {
                            // we are pushing updateLowLinks first so it will run *after* findComponent finishes
                            stack.push((current_node_index, State::UpdateLowLinks(adj_node)));
                            stack.push((adj_node, State::FindComponent));
                        }
                    }
                }
                State::FindComponent => {
                    self.add_node(current_node_index);

                    stack.push((current_node_index, State::BuildComponent)); // we will come back to this once all others are explored

                    graph.do_for_all_neighbors(current_node_index, false, |adj_node| {
                        stack.push((current_node_index, State::HandleNeighbor(adj_node)));
                    });
                }
            }
        }
    }

    fn add_node(&mut self, node_index: usize) {
        if let None = self.nodes[node_index] {
            self.nodes[node_index] = Some(Box::new(AlgorithmNode {
                index: self.index,
                low_link: self.index,
                on_stack: true,
            }));

            self.index += 1;
            self.stack.push(node_index);
        }
    }
}

impl<G: Graph> ComponentsAlgorithm<G> for TarjanComponentsAlgorithm2 {
    fn get_components(&self, graph: &G) -> Vec<HashSet<usize>> {
        let mut algorithm_data = AlgorithmData::new(graph.get_nr_nodes());

        for i in 0..graph.get_nr_nodes() {
            if algorithm_data.nodes[i].is_none() {
                //algorithm_data.strongconnect_recursive(graph, i);
                algorithm_data.strongconnect_iterative(graph, i);
            }
        }

        algorithm_data.components
    }
}
