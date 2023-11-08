use super::heapentry::*;
use super::Path;
use super::RoutingResult;
use super::options::RoutingAlgorithm;
use crate::core::Graph;
use crate::core::WeightCalculator;

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::vec;

pub struct DijkstraRoutingAlgorithm {
    pub path: bool,
    pub weight_calculator: WeightCalculator,
}

struct AlgorithmData {
    distances: HashMap<i32, f64>,
    used: HashSet<i32>,
    heap: BinaryHeap<Rc<HeapEntry>>,
    current_heap_entry: Rc<HeapEntry>,
}

fn init_algorithm_data(start: i32) -> AlgorithmData {
    let mut distances: HashMap<i32, f64> = HashMap::new();
    distances.insert(start, 0.0);

    let used = HashSet::new();
    let mut heap = BinaryHeap::new();

    let current_heap_entry = Rc::new(HeapEntry::new(0.0, start, None, None));
    heap.push(Rc::clone(&current_heap_entry));

    AlgorithmData {
        distances,
        used,
        heap,
        current_heap_entry,
    }
}

impl<G:Graph> RoutingAlgorithm<G> for DijkstraRoutingAlgorithm {
    fn route(&self, graph: &G, start: i32, end: i32) -> Option<RoutingResult> {
        let AlgorithmData {
            mut distances,
            mut used,
            mut heap,
            mut current_heap_entry,
        } = init_algorithm_data(start);

        while !heap.is_empty() && !used.contains(&end) {
            current_heap_entry = heap.pop().unwrap(); //OK because of is_empty check above
            let index = current_heap_entry.value;

            //Since we are only pushing on the heap entry and not updating existing values
            //it is possible that we do the same node twice. But theres no point in redoing a settled node
            if !used.insert(index) {
                continue;
            }

            if index == end {
                break;
            }

            graph.do_for_all_neighbors(index, false, |adj_node, edge| {
                if !used.contains(&adj_node) {
                    //if dist(start->index) + dist(index->adj_node) < dist(start->adj_node)
                    let dist1 = *distances.get(&index).unwrap_or(&f64::INFINITY);
                    let weight = &self.weight_calculator.calc_weight(edge, index);
                    let dist2 = distances.entry(adj_node).or_insert(f64::INFINITY);
                    if dist1 + weight < *dist2 {
                        *dist2 = dist1 + weight;

                        let mut parent = None;
                        let mut edge_info = None;
                        if self.path {
                            parent = Some(Rc::clone(&current_heap_entry));
                            let edge_entry = Some(Rc::clone(edge));
                            edge_info = create_edge_information(graph, edge_entry, index, adj_node, false);
                        }
                        let new_heap_entry = Rc::new(HeapEntry::new(*dist2, adj_node, edge_info , parent));
                        heap.push(new_heap_entry);
                    }
                }
            });
        }

        if !used.contains(&end) {
            None
        } else {
            Some(RoutingResult {
                distance: *current_heap_entry.key,
                weight: *current_heap_entry.key,
                paths: vec![Path::new(current_heap_entry.get_path(true))],
            })
        }
    }
}
