use perionav::core::{
    routing::options::{AlgorithmType, RoutingAlgorithmOptions, WeightType},
    routing::RoutingResult,
    Graph,
};
use rstest::rstest;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

mod create_graph;

fn assert_route_weight(result: &Option<RoutingResult>, weight: f64) {
    assert!(!result.is_none());
    assert_eq!(result.as_ref().unwrap().weight, weight);
}

fn assert_route_path(result: &Option<RoutingResult>, nodes: Vec<i32>) {
    assert!(!result.is_none());
    let result = result.as_ref().unwrap();
    assert!(!result.paths.is_empty());
    assert_eq!(result.paths.get(0).unwrap().get_nodes(), nodes);
}

#[rstest]
fn test_disconnected_graph(#[values(AlgorithmType::DIJKSTRA, AlgorithmType::DIJKSTRA2, AlgorithmType::BIDIRDIJKSTRA)] algorithm_type: AlgorithmType) {
    let g = create_graph::create_ii_graph();
    let opts = RoutingAlgorithmOptions::new(true, algorithm_type, WeightType::DISTANCE);

    assert_route_weight(&g.route(&opts, 0, 1), 1.0);
    assert!(g.route(&opts, 0, 2).is_none());
}

#[rstest]
fn test_connected_graph(#[values(AlgorithmType::DIJKSTRA, AlgorithmType::DIJKSTRA2, AlgorithmType::BIDIRDIJKSTRA)] algorithm_type: AlgorithmType) {
    let g = create_graph::create_k3_graph();
    let opts = RoutingAlgorithmOptions::new(true, algorithm_type, WeightType::DISTANCE);

    assert_route_weight(&g.route(&opts, 0, 1), 1.0);
    assert_route_weight(&g.route(&opts, 0, 2), 1.0);
}

#[rstest]
fn test_square_graph(#[values(AlgorithmType::DIJKSTRA, AlgorithmType::DIJKSTRA2, AlgorithmType::BIDIRDIJKSTRA)] algorithm_type: AlgorithmType) {
    let g = create_graph::create_square_graph();
    let opts = RoutingAlgorithmOptions::new(true, algorithm_type, WeightType::DISTANCE);

    assert_route_weight(&g.route(&opts, 0, 1), 1.0);
    assert_route_weight(&g.route(&opts, 0, 2), 2.0);
}

#[rstest]
fn test_line_graph(#[values(AlgorithmType::DIJKSTRA, AlgorithmType::DIJKSTRA2, AlgorithmType::BIDIRDIJKSTRA)] algorithm_type: AlgorithmType) {
    let g = create_graph::create_line_graph();
    let opts = RoutingAlgorithmOptions::new(true, algorithm_type, WeightType::DISTANCE);

    let result = &g.route(&opts, 0, 1);
    assert_route_weight(result, 1.0);
    assert_route_path(&result, vec![0, 1]);
}

#[rstest]
fn test_complex_graph(#[values(AlgorithmType::DIJKSTRA, AlgorithmType::DIJKSTRA2, AlgorithmType::BIDIRDIJKSTRA)] algorithm_type: AlgorithmType) {
    let g = create_graph::create_complex_graph();
    let opts = RoutingAlgorithmOptions::new(true, algorithm_type, WeightType::DISTANCE);

    let result = g.route(&opts, 3, 6);
    assert_route_weight(&result, 8.0);
    assert_route_path(&result, vec![3, 1, 4, 0, 6]);
}

#[test]
fn test_different_algorithms_equal() {
    let nodes = 1000;
    let g = create_graph::create_random_graph(nodes, 3000);

    let opts = RoutingAlgorithmOptions::new(true, AlgorithmType::DIJKSTRA, WeightType::DISTANCE);
    let opts2 = RoutingAlgorithmOptions::new(true, AlgorithmType::DIJKSTRA2, WeightType::DISTANCE);
    let opts3 = RoutingAlgorithmOptions::new(true, AlgorithmType::BIDIRDIJKSTRA, WeightType::DISTANCE);

    let mut rng = StdRng::seed_from_u64(42);

    for i in 1..1000 {
        let from = rng.gen_range(0..nodes);
        let to = rng.gen_range(0..nodes);

        let r1 = g.route(&opts, from, to);
        let r2 = g.route(&opts2, from, to);
        let r3 = g.route(&opts3, from, to);

        assert!(result_equal(&r1, &r2), "r1!=r2 for test case {}. Routing from {} to {}", i, from, to);
        assert!(result_equal(&r2, &r3), "r2!=r3 for test case {}. Routing from {} to {}", i, from, to);
    }
}

fn result_equal(r1: &Option<RoutingResult>, r2: &Option<RoutingResult>) -> bool {
    if r1.is_none() {
        return r2.is_none();
    }

    let result1 = r1.as_ref().unwrap();
    let result2 = r2.as_ref().unwrap();

    return delta_equal(result1.weight, result2.weight, 1E-7);
}

fn delta_equal(f1: f64, f2: f64, delta: f64) -> bool {
    (f1 - f2).abs() < delta
}
