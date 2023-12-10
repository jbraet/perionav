use std::{collections::{HashSet, HashMap, hash_map::Entry}, cmp};

use crate::core::Graph;

use super::options::ComponentsAlgorithm;

pub struct TarjanComponentsAlgorithm {}

struct AlgorithmNode {
    index: usize, // the index in which order the graph is explored
    low_link: usize,
    on_stack: bool,
}

struct AlgorithmData {
    index: usize,
    stack: Vec<usize>,

    nodes: HashMap<usize, AlgorithmNode>,
    components: Vec<HashSet<i32>>, //actual return value
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

    fn strongconnect_recursive(&mut self, graph: &impl Graph, node_index: usize) {
        match self.nodes.entry(node_index) {
            Entry::Vacant(entry) => {
                entry.insert(
                    AlgorithmNode{
                        index: self.index,
                        low_link: self.index,
                        on_stack: true,
                    }
                )
            }
            Entry::Occupied(_) => {
                return //means this node was explored before already
            }
        };

        self.index+=1;

        self.stack.push(node_index);
        
        graph.do_for_all_neighbors(node_index as i32, false, |adj_node| {
            if !self.nodes.contains_key(&(adj_node as usize)) {
                self.strongconnect_recursive(graph, adj_node as usize);

                let w_low_link = self.nodes.get(&(adj_node as usize)).unwrap().low_link; 
                let v = self.nodes.get_mut(&node_index).unwrap();
                
                v.low_link = cmp::min(v.low_link, w_low_link);
            } else {
                let w = self.nodes.get(&(adj_node as usize)).unwrap();
                let w_index = w.index;
                let w_on_stack = w.on_stack;

                if w_on_stack {
                    let v = self.nodes.get_mut(&node_index).unwrap();

                    v.low_link = cmp::min(v.low_link, w_index);
                }
            }
        });

        //we have to get v again because it might have changed in the meanwhile
        let v = self.nodes.get(&node_index).unwrap(); 
        if v.low_link == v.index {
            let mut component = HashSet::new();

            while self.stack.pop().is_some_and(|w_index|{
                self.nodes.entry(w_index).and_modify(|w| {
                    w.on_stack = false;
                }); //if it doesn't exist then there is nothing to do, also it shouldn't happen
                
                component.insert(w_index as i32);
                w_index!=node_index
            }) {}

            self.components.push(component);
        }

    }
}

impl<G:Graph> ComponentsAlgorithm<G> for TarjanComponentsAlgorithm {
    fn get_components(&self, graph: &G) -> Vec<HashSet<i32>> {

        let mut algorithm_data = AlgorithmData::new();

        for i in 0..graph.get_nr_nodes() {
            algorithm_data.strongconnect_recursive(graph, i); //the func will return immediately if we don't have to do it anymore
        }
        
        algorithm_data.components
    }
}