use super::{bidirdijkstra::BidirDijkstraRoutingAlgorithm, dijkstra::DijkstraRoutingAlgorithm, dijkstra2::DijkstraRoutingAlgorithm2, RoutingResult};
use crate::core::weight::{DistanceWeight, TravelTimeWeight};
use crate::core::Graph;
use crate::core::WeightCalculator;

#[non_exhaustive]
pub enum AlgorithmType {
    DIJKSTRA,
    DIJKSTRA2,
    BIDIRDIJKSTRA,
}

#[non_exhaustive]
pub enum WeightType {
    DISTANCE,
    TRAVELTIME,
}

/// Plan a route given a start and end node
/// start and end must be a valid node within graph
pub trait RoutingAlgorithm<G: Graph> {
    fn route(&self, graph: &G, start: usize, end: usize) -> Option<RoutingResult>;
}

pub struct RoutingAlgorithmOptions<G: Graph> {
    //path: bool,
    //algorithm_type: AlgorithmType,
    //weight_type: WeightType,
    pub routing_algorithm: Box<dyn RoutingAlgorithm<G>>,
}

impl<G: Graph> RoutingAlgorithmOptions<G> {
    //path: keep track of a path or not
    pub fn new(path: bool, algorithm_type: AlgorithmType, weight_type: WeightType) -> Self {
        let routing_algorithm = create_routing_algorithm(path, &algorithm_type, &weight_type);
        RoutingAlgorithmOptions {
            //path,
            //algorithm_type,
            //weight_type,
            routing_algorithm,
        }
    }
}

pub fn create_weight_calculator(weight_type: &WeightType) -> WeightCalculator {
    match weight_type {
        WeightType::DISTANCE => WeightCalculator::TravelTime(TravelTimeWeight {}),
        WeightType::TRAVELTIME => WeightCalculator::Distance(DistanceWeight {}),
    }
}

// Creates a routing algorithm based on the given algorithm options
pub fn create_routing_algorithm<G: Graph>(path: bool, algorithm_type: &AlgorithmType, weight_type: &WeightType) -> Box<dyn RoutingAlgorithm<G>> {
    let weight_calculator = create_weight_calculator(weight_type);
    match algorithm_type {
        AlgorithmType::DIJKSTRA => Box::new(DijkstraRoutingAlgorithm { path, weight_calculator }),
        AlgorithmType::DIJKSTRA2 => Box::new(DijkstraRoutingAlgorithm2 { path, weight_calculator }),
        AlgorithmType::BIDIRDIJKSTRA => Box::new(BidirDijkstraRoutingAlgorithm { path, weight_calculator }),
    }
}
