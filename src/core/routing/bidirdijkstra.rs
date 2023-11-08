#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use super::heapentry::*;
use super::Path;
use super::RoutingResult;
use crate::core::routingoptions::RoutingAlgorithm;
use crate::core::Graph;
use crate::core::WeightCalculator;

use ordered_float::NotNan;

use std::borrow::BorrowMut;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::vec;

use crate::core::edgeinformation::EdgeInformation;

pub struct BidirDijkstraRoutingAlgorithm {
    pub path: bool,
    pub weight_calculator: WeightCalculator,
}

struct SingleDirectionAlgorithmData {
    distances: HashMap<i32, Rc<HeapEntry>>,
    used: HashSet<i32>,
    heap: BinaryHeap<Rc<HeapEntry>>,
    heap_entry: Rc<HeapEntry>, //the current top of the heap that we are considering
    end: i32,
}

struct AlgorithmData {
    forward: SingleDirectionAlgorithmData,
    backward: SingleDirectionAlgorithmData,
    best: BestData,
}

struct BestData {
    weight: f64,
    fwd_entry: Option<Rc<HeapEntry>>,
    bwd_entry: Option<Rc<HeapEntry>>,
}

fn init_algorithm_data(start: i32, end: i32) -> AlgorithmData {
    let mut forward_heap = BinaryHeap::new();
    let mut backward_heap = BinaryHeap::new();

    let mut forward_heap_entry = Rc::new(HeapEntry::new(0.0, start, None, None));
    forward_heap.push(Rc::clone(&forward_heap_entry));

    let mut backward_heap_entry = Rc::new(HeapEntry::new(0.0, end, None, None));
    backward_heap.push(Rc::clone(&backward_heap_entry));

    let mut forward_distances: HashMap<i32, Rc<HeapEntry>> = HashMap::new();
    forward_distances.insert(start, Rc::clone(&forward_heap_entry));
    let mut backward_distances: HashMap<i32, Rc<HeapEntry>> = HashMap::new();
    backward_distances.insert(end, Rc::clone(&backward_heap_entry));

    let used_forward = HashSet::new();
    let used_backward = HashSet::new();

    let best = if start==end { //special case: routing to the same node needs a 0 weight result.
        BestData { 
            weight: 0.0,
            fwd_entry: Some(Rc::clone(&forward_heap_entry)),
            bwd_entry: Some(Rc::clone(&backward_heap_entry)),
        }
    } else {
        BestData { 
            weight: f64::INFINITY,
            fwd_entry: None,
            bwd_entry: None,
        }
    };

    AlgorithmData {
        forward: SingleDirectionAlgorithmData {
            distances: forward_distances,
            used: used_forward,
            heap: forward_heap,
            heap_entry: forward_heap_entry,
            end,
        },
        backward: SingleDirectionAlgorithmData {
            distances: backward_distances,
            used: used_backward,
            heap: backward_heap,
            heap_entry: backward_heap_entry,
            end: start,
        },
        best,
    }
}

impl BidirDijkstraRoutingAlgorithm {
    //returns if the forward direction is finished
    fn route_forward(&self, graph: &Graph, data: &mut AlgorithmData) -> bool {
        self.fill_edges(graph, &mut data.forward, &data.backward, &mut data.best, false)
    }

    fn route_backward(&self, graph: &Graph, data: &mut AlgorithmData) -> bool {
        self.fill_edges(graph, &mut data.backward, &data.forward, &mut data.best, true)
    }

    fn fill_edges(&self, graph: &Graph, data: &mut SingleDirectionAlgorithmData, other_data: &SingleDirectionAlgorithmData, best: &mut BestData, reverse: bool) -> bool {
        while !data.heap.is_empty() && !data.used.contains(&data.end) {
            data.heap_entry = data.heap.pop().unwrap(); //OK because of is_empty check above
            let index = data.heap_entry.value;
            let dist1 = data.heap_entry.key;

            //Since we are only pushing on the heap entry and not updating existing values
            //it is possible that we do the same node twice. But theres no point in redoing a settled node
            if !data.used.insert(index) {
                continue;
            }

            if index == data.end {
                return true;
            }

            graph.do_for_all_neighbors(index, reverse, |adj_node, edge| {
                if !data.used.contains(&adj_node) {
                    //if dist(start->index) + dist(index->adj_node) < dist(start->adj_node)
                    let dist1 = *data.distances.get(&index).map_or(&f64::INFINITY, |heap_entry| &heap_entry.key);

                    let weight = &self.weight_calculator.calc_weight(edge, if !reverse {index} else {adj_node});

                    let mut create_new_heap_entry = || {
                        let mut parent = None;
                        let mut edge_info = None;
                        if self.path {
                            parent = Some(Rc::clone(&data.heap_entry));
                            let edge_entry = Some(Rc::clone(edge));
                            edge_info = create_edge_information(edge_entry, index, adj_node, reverse);
                        }

                        let ret = Rc::new(HeapEntry::new(dist1 + weight, adj_node, edge_info, parent));

                        data.heap.push(Rc::clone(&ret));

                        ret
                    };

                    let dist2 = data.distances.entry(adj_node).and_modify(|e| {
                        if dist1+weight < e.key.into_inner() {
                            *e = create_new_heap_entry();
                        }
                    }).or_insert_with(create_new_heap_entry);

                    let other_heap_entry = other_data.distances.get(&adj_node);
                    let other_dist = other_heap_entry.map_or(&f64::INFINITY, |heap_entry| &heap_entry.key);
                    if dist1+weight+other_dist < best.weight && other_data.used.contains(&adj_node) {
                        let other_heap_entry_unwrapped = other_heap_entry.unwrap(); // safe because otherwise other_dist will be infite and will never satisfy the above condition
                        best.weight = dist1+weight+other_dist;
                        if !reverse {
                            best.fwd_entry = Some(Rc::clone(&data.heap_entry));
                            best.bwd_entry = Some(Rc::clone(other_heap_entry_unwrapped));
                        } else {
                            best.fwd_entry = Some(Rc::clone(other_heap_entry_unwrapped));
                            best.bwd_entry = Some(Rc::clone(&data.heap_entry));
                        }
                    }
                }
            });

            return false;
        }

        true
    }
}

impl RoutingAlgorithm for BidirDijkstraRoutingAlgorithm {
    fn route(&self, graph: &Graph, start: i32, end: i32) -> Option<RoutingResult> {
        let mut data = init_algorithm_data(start, end);

        let mut finished_fwd = false;
        let mut finished_bwd = false;

        while !finished_fwd && !finished_bwd
            && *data.forward.heap_entry.key + *data.backward.heap_entry.key < data.best.weight
        {
            finished_fwd = self.route_forward(graph, &mut data);
            finished_bwd = self.route_backward(graph, &mut data);
        }

        if data.best.weight==f64::INFINITY {
            None
        } else {
            Some(RoutingResult {
                distance: data.best.weight,
                weight: data.best.weight,
                paths: vec![extract_path(graph, data.best.fwd_entry, data.best.bwd_entry, start, end)],
            })
        }
    }
}

fn extract_path(graph: &Graph, fwd: Option<Rc<HeapEntry>>, bwd: Option<Rc<HeapEntry>>, start: i32, end: i32) -> Path {
    let (fwd_edges,fwd_last_node) = match fwd {
        None => (vec![],start),
        Some(fwd_entry) => (fwd_entry.get_path(true),fwd_entry.value),
    };
    let mut path = Path::new(fwd_edges);

    let (bwd_edges, bwd_first_node) = match bwd {
        None => (vec![],end),
        Some(bwd_entry) => (bwd_entry.get_path(false),bwd_entry.value),
    };

    if fwd_last_node!=bwd_first_node {
        let middle_edge_option = graph.get_directed_edge_between(fwd_last_node, bwd_first_node);
        if let Some(middle_edge) = middle_edge_option {
            path.add_edges(vec![middle_edge]);
        }
    }

    path.add_edges(bwd_edges);

    path
}