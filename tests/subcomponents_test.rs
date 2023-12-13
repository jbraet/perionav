use std::collections::HashSet;

use perionav::core::{
    components::options::{AlgorithmType, ComponentsAlgorithmOptions},
    Graph,
};
use rstest::rstest;

mod create_graph;

fn assert_component_sizes(components: Vec<HashSet<i32>>, expected: Vec<usize>) {
    let mut sizes = vec![];
    for component in components {
        sizes.push(component.len())
    }
    sizes.sort_unstable();

    assert_eq!(sizes, expected);
}

fn assert_components_equal(components1: &Vec<HashSet<i32>>, components2: &Vec<HashSet<i32>>) {
    let mut sizes1 = vec![];
    for component in components1 {
        sizes1.push(component.len())
    }
    sizes1.sort_unstable();

    let mut sizes2 = vec![];
    for component in components2 {
        sizes2.push(component.len())
    }
    sizes2.sort_unstable();

    assert_eq!(sizes1, sizes2);
}

#[rstest]
fn test_small_sub_components_graph(
    #[values(AlgorithmType::PATHBASED, AlgorithmType::KOSARAJU, AlgorithmType::TARJAN)] algorithm_type: AlgorithmType,
) {
    let g = create_graph::create_small_sub_components_graph();
    let opts = ComponentsAlgorithmOptions::new(algorithm_type);

    let components = g.get_strongly_connected_subgraphs(&opts);
    assert_component_sizes(components, vec![2, 3]);
}

#[rstest]
fn test_sub_components_graph(#[values(AlgorithmType::PATHBASED, AlgorithmType::KOSARAJU, AlgorithmType::TARJAN)] algorithm_type: AlgorithmType) {
    let g = create_graph::create_sub_components_graph();
    let opts = ComponentsAlgorithmOptions::new(algorithm_type);

    let components = g.get_strongly_connected_subgraphs(&opts);
    assert_component_sizes(components, vec![1, 2, 2, 3]);
}

#[rstest]
fn test_disconnected_graph(#[values(AlgorithmType::PATHBASED, AlgorithmType::KOSARAJU, AlgorithmType::TARJAN)] algorithm_type: AlgorithmType) {
    let g = create_graph::create_ii_graph();
    let opts = ComponentsAlgorithmOptions::new(algorithm_type);

    let components = g.get_strongly_connected_subgraphs(&opts);
    assert_component_sizes(components, vec![2, 2]);
}

#[rstest]
fn test_semi_connected_graph(#[values(AlgorithmType::PATHBASED, AlgorithmType::KOSARAJU, AlgorithmType::TARJAN)] algorithm_type: AlgorithmType) {
    let g = create_graph::create_connected_ii_graph();
    let opts = ComponentsAlgorithmOptions::new(algorithm_type);

    let components = g.get_strongly_connected_subgraphs(&opts);
    assert_component_sizes(components, vec![2, 2]);
}

#[rstest]
fn test_square_graph(#[values(AlgorithmType::PATHBASED, AlgorithmType::KOSARAJU, AlgorithmType::TARJAN)] algorithm_type: AlgorithmType) {
    let g = create_graph::create_square_graph();
    let opts = ComponentsAlgorithmOptions::new(algorithm_type);

    let components = g.get_strongly_connected_subgraphs(&opts);
    assert_component_sizes(components, vec![4]);
}

#[rstest]
fn test_legs_graph(#[values(AlgorithmType::PATHBASED, AlgorithmType::KOSARAJU, AlgorithmType::TARJAN)] algorithm_type: AlgorithmType) {
    let g = create_graph::create_legs_graph();
    let opts = ComponentsAlgorithmOptions::new(algorithm_type);

    let components = g.get_strongly_connected_subgraphs(&opts);
    assert_component_sizes(components, vec![1, 2]);
}

#[rstest]
fn test_diamond_graph(#[values(AlgorithmType::PATHBASED, AlgorithmType::KOSARAJU, AlgorithmType::TARJAN)] algorithm_type: AlgorithmType) {
    let g = create_graph::create_diamond_graph();
    let opts = ComponentsAlgorithmOptions::new(algorithm_type);

    let components = g.get_strongly_connected_subgraphs(&opts);
    assert_component_sizes(components, vec![1, 1, 1, 1]);
}

#[rstest]
fn test_strongly_disconnected_graph(
    #[values(AlgorithmType::PATHBASED, AlgorithmType::KOSARAJU, AlgorithmType::TARJAN)] algorithm_type: AlgorithmType,
) {
    let g = create_graph::create_strongly_disconnected_graph();
    let opts = ComponentsAlgorithmOptions::new(algorithm_type);

    let components = g.get_strongly_connected_subgraphs(&opts);
    assert_component_sizes(components, vec![1, 1, 1]);
}

#[test]
fn test_algorithms_equal() {
    let nodes = 3000;
    let g = create_graph::create_random_graph(nodes, nodes * 2);

    let opts = ComponentsAlgorithmOptions::new(AlgorithmType::PATHBASED);
    let opts2 = ComponentsAlgorithmOptions::new(AlgorithmType::KOSARAJU);
    let opts3 = ComponentsAlgorithmOptions::new(AlgorithmType::TARJAN);

    let components = g.get_strongly_connected_subgraphs(&opts);
    let components2 = g.get_strongly_connected_subgraphs(&opts2);
    let components3 = g.get_strongly_connected_subgraphs(&opts3);

    assert_components_equal(&components, &components2);
    assert_components_equal(&components2, &components3);
}
