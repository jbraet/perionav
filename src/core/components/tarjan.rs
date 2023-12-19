use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt,
};

use crate::core::Graph;

use super::options::ComponentsAlgorithm;

pub struct TarjanComponentsAlgorithm {}

#[derive(Debug)]
struct AlgorithmNode {
    index: usize, // the index in which order the graph is explored
    low_link: usize,
    on_stack: bool,
}

struct AlgorithmData {
    index: usize,
    stack: Vec<usize>,

    nodes: HashMap<usize, AlgorithmNode>,
    components: Vec<HashSet<usize>>, //actual return value
}

enum State {
    Initial,
    SingleNeighborVisited(usize), //after visiting a single neighbor we have to do an update function based on the neighbor visited
    AllNeighborsVisited,
    UpdateLowLinks(usize),
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Initial => write!(f, "initial"),
            State::SingleNeighborVisited(adj) => write!(f, "singleVisited {}", adj),
            State::AllNeighborsVisited => write!(f, "allVisited"),
            State::UpdateLowLinks(adj) => write!(f, "updateLowLinks {}", adj),
        }
    }
}

impl AlgorithmData {
    fn new() -> Self {
        AlgorithmData {
            index: 0,
            stack: vec![],
            nodes: HashMap::new(),
            components: vec![],
        }
    }

    #[allow(dead_code)]
    fn strongconnect_recursive(&mut self, graph: &impl Graph, node_index: usize) {
        match self.nodes.entry(node_index) {
            Entry::Vacant(entry) => {
                entry.insert(AlgorithmNode {
                    index: self.index,
                    low_link: self.index,
                    on_stack: true,
                });
                self.index += 1;
                self.stack.push(node_index);
            }
            Entry::Occupied(_) => {
                return; //means this node was explored before already
            }
        };

        graph.do_for_all_neighbors(node_index, false, |adj_node| {
            if !self.nodes.contains_key(&(adj_node)) {
                self.strongconnect_recursive(graph, adj_node);

                let w_low_link = self.nodes.get(&(adj_node)).unwrap().low_link;
                let v = self.nodes.get_mut(&node_index).unwrap();

                if w_low_link < v.low_link {
                    v.low_link = w_low_link;
                }
            } else {
                let w = self.nodes.get(&(adj_node)).unwrap();
                let w_index = w.index;
                let w_on_stack = w.on_stack;

                if w_on_stack {
                    let v = self.nodes.get_mut(&node_index).unwrap();

                    if w_index < v.low_link {
                        v.low_link = w_index;
                    }
                }
            }
        });

        //we have to get v again because it might have changed in the meanwhile
        let v = self.nodes.get(&node_index).unwrap();
        if v.low_link == v.index {
            let mut component = HashSet::new();

            let mut test_function = |w_index| {
                self.nodes.entry(w_index).and_modify(|w| {
                    w.on_stack = false;
                }); //if it doesn't exist then there is nothing to do, also it shouldn't happen

                component.insert(w_index);
                w_index != node_index
            };
            while self.stack.pop().is_some_and(&mut test_function) {}

            self.components.push(component);
        }
    }

    fn strongconnect_iterative(&mut self, graph: &impl Graph, node_index: usize) {
        let mut stack: Vec<(usize, State)> = Vec::new();
        stack.push((node_index, State::Initial));

        while let Some((current_node_index, visit_neighbors)) = stack.pop() {
            match visit_neighbors {
                State::Initial => {
                    if self.add_node(current_node_index) {
                        continue;
                    }

                    stack.push((current_node_index, State::AllNeighborsVisited)); // we will come back to this once all others are explored

                    graph.do_for_all_neighbors(current_node_index, false, |adj_node| {
                        if !self.nodes.contains_key(&(adj_node)) {
                            stack.push((current_node_index, State::SingleNeighborVisited(adj_node)));
                            stack.push((adj_node, State::Initial));
                        } else {
                            stack.push((current_node_index, State::UpdateLowLinks(adj_node)));
                        }
                    });
                }
                State::AllNeighborsVisited => {
                    let v = self.nodes.get(&current_node_index).unwrap();
                    if v.low_link == v.index {
                        let mut component = HashSet::new();

                        let mut test_function = |w_index| {
                            self.nodes.entry(w_index).and_modify(|w| {
                                w.on_stack = false;
                            }); //if it doesn't exist then there is nothing to do, also it shouldn't happen

                            component.insert(w_index);
                            w_index != current_node_index
                        };
                        while self.stack.pop().is_some_and(&mut test_function) {}

                        self.components.push(component);
                    }
                }
                State::SingleNeighborVisited(adj_node) => {
                    let w_low_link = self.nodes.get(&adj_node).unwrap().low_link;
                    let v = self.nodes.get_mut(&current_node_index).unwrap();

                    if w_low_link < v.low_link {
                        v.low_link = w_low_link;
                    }
                }
                State::UpdateLowLinks(adj_node) => {
                    let w = self.nodes.get(&adj_node).unwrap();
                    let w_index = w.index;
                    let w_on_stack = w.on_stack;

                    if w_on_stack {
                        let v = self.nodes.get_mut(&current_node_index).unwrap();

                        if w_index < v.low_link {
                            v.low_link = w_index;
                        }
                    }
                }
            }
        }
    }

    fn add_node(&mut self, node_index: usize) -> bool {
        match self.nodes.entry(node_index) {
            Entry::Vacant(entry) => {
                entry.insert(AlgorithmNode {
                    index: self.index,
                    low_link: self.index,
                    on_stack: true,
                });

                self.index += 1;
                self.stack.push(node_index);

                false
            }
            Entry::Occupied(_) => {
                true //means this node was explored before already
            }
        }
    }
}

impl<G: Graph> ComponentsAlgorithm<G> for TarjanComponentsAlgorithm {
    fn get_components(&self, graph: &G) -> Vec<HashSet<usize>> {
        let mut algorithm_data = AlgorithmData::new();

        for i in 0..graph.get_nr_nodes() {
            if !algorithm_data.nodes.contains_key(&i) {
                //algorithm_data.strongconnect_recursive(graph, i);
                algorithm_data.strongconnect_iterative(graph, i);
            }
        }

        algorithm_data.components
    }
}
