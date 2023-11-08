mod create_graph;

#[test]
fn test_disconnected_graph() {
    let g = create_graph::create_ii_graph();

    assert!(!g.is_strongly_connected());
}

#[test]
fn test_connected_graph() {
    let g = create_graph::create_k3_graph();

    assert!(g.is_strongly_connected());
}

#[test]
fn test_square_graph() {
    let g = create_graph::create_square_graph();

    assert!(g.is_strongly_connected());
}

#[test]
fn test_complex_graph() {
    let g = create_graph::create_complex_graph();

    assert!(g.is_strongly_connected());
}
