use super::heapentry2::*;
use super::Path;
use super::RoutingResult;
use super::options::RoutingAlgorithm;
use crate::core::Graph;
use crate::core::WeightCalculator;

use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::rc::Rc;
use std::vec;

pub struct DijkstraRoutingAlgorithm2 {
    pub path: bool,
    pub weight_calculator: WeightCalculator,
}

//we need to use refcell because otherwise we aren't able to change the deleted field inside the Rc<HeapEntry>
struct AlgorithmData {
    distances: HashMap<i32, Rc<RefCell<HeapEntry>>>,
    heap: BinaryHeap<Rc<RefCell<HeapEntry>>>,
    current_heap_entry: Rc<RefCell<HeapEntry>>,
}

fn init_algorithm_data(graph: &impl Graph, start: i32) -> AlgorithmData {
    let mut distances: HashMap<i32, Rc<RefCell<HeapEntry>>> = HashMap::new();
    let mut heap = BinaryHeap::new();

    let current_heap_entry = Rc::new(RefCell::new(HeapEntry::new(graph, 0.0, start, None, None)));
    heap.push(Rc::clone(&current_heap_entry));
    distances.insert(start, Rc::clone(&current_heap_entry));

    AlgorithmData {
        distances,
        heap,
        current_heap_entry,
    }
}

impl <G:Graph> RoutingAlgorithm<G> for DijkstraRoutingAlgorithm2 {
    fn route(&self, graph: &G, start: i32, end: i32) -> Option<RoutingResult> {
        let AlgorithmData {
            mut distances,
            mut heap,
            mut current_heap_entry,
        } = init_algorithm_data(graph, start);

        while !heap.is_empty() {
            current_heap_entry = heap.pop().unwrap(); //OK because of is_empty check above
            let current_heap_entry_borrowed = current_heap_entry.borrow();
            if current_heap_entry_borrowed.deleted {
                continue;
            }

            let index = current_heap_entry_borrowed.value;
            if index == end {
                break;
            }

            graph.do_for_all_neighbors(index, false, |adj_node, edge| {
                let adj_heap_entry = distances.get(&adj_node);

                let mut parent = None;
                let mut edge_entry = None;
                if self.path {
                    parent = Some(Rc::clone(&current_heap_entry));
                    edge_entry = Some(Rc::clone(edge));
                }

                let weight = &self.weight_calculator.calc_weight(edge, index);
                let dist2 = *current_heap_entry_borrowed.key + weight;
                match adj_heap_entry {
                    None => {
                        let new_heap_entry = Rc::new(RefCell::new(HeapEntry::new(graph,
                            dist2, adj_node, edge_entry, parent,
                        )));
                        heap.push(Rc::clone(&new_heap_entry));
                        distances.insert(adj_node, Rc::clone(&new_heap_entry));
                    }
                    Some(adj_heap_entry) => {
                        if *adj_heap_entry.borrow().key > dist2 {
                            adj_heap_entry.borrow_mut().deleted = true;
                            let new_heap_entry = Rc::new(RefCell::new(HeapEntry::new(graph,
                                dist2, adj_node, edge_entry, parent,
                            )));
                            heap.push(new_heap_entry);
                        } //else do nothing
                    }
                };
            });
        }

        if !distances.contains_key(&end) {
            None
        } else {
            Some(RoutingResult {
                distance: *current_heap_entry.borrow().key,
                weight: *current_heap_entry.borrow().key,
                paths: vec![Path::new(current_heap_entry.borrow().get_path())],
            })
        }
    }
}
