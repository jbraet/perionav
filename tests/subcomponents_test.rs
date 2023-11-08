use std::collections::HashSet;

use perionav::core::{
    Graph,
    components::options::{ComponentsAlgorithmOptions, AlgorithmType},
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

#[rstest]
fn test_small_sub_components_graph(#[values(AlgorithmType::PATHBASED)] algorithm_type:AlgorithmType) {
    let g = create_graph::create_small_sub_components_graph();
    let opts = ComponentsAlgorithmOptions::new(algorithm_type);

    let components = g.get_strongly_connected_subgraphs(&opts);
    assert_component_sizes(components, vec![2,3]);
}

#[rstest]
fn test_sub_components_graph(#[values(AlgorithmType::PATHBASED)] algorithm_type:AlgorithmType) {
    let g = create_graph::create_sub_components_graph();
    let opts = ComponentsAlgorithmOptions::new(algorithm_type);

    let components = g.get_strongly_connected_subgraphs(&opts);
    assert_component_sizes(components, vec![1,2,2,3]);
}

#[rstest]
fn test_disconnected_graph(#[values(AlgorithmType::PATHBASED)] algorithm_type:AlgorithmType) {
    let g = create_graph::create_ii_graph();
    let opts = ComponentsAlgorithmOptions::new(algorithm_type);

    let components = g.get_strongly_connected_subgraphs(&opts);
    assert_component_sizes(components, vec![2,2]);
}

#[rstest]
fn test_semi_connected_graph(#[values(AlgorithmType::PATHBASED)] algorithm_type:AlgorithmType) {
    let g = create_graph::create_connected_ii_graph();
    let opts = ComponentsAlgorithmOptions::new(algorithm_type);

    let components = g.get_strongly_connected_subgraphs(&opts);
    assert_component_sizes(components, vec![2,2]);
}

#[rstest]
fn test_square_graph(#[values(AlgorithmType::PATHBASED)] algorithm_type:AlgorithmType) {
    let g = create_graph::create_square_graph();
    let opts = ComponentsAlgorithmOptions::new(algorithm_type);

    let components = g.get_strongly_connected_subgraphs(&opts);
    assert_component_sizes(components, vec![4]);
}