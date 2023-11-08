use perionav::core::edge::Edge;
use perionav::core::Graph;
use perionav::core::StandardGraph;
use perionav::core::node::Node;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn create_random_graph(nodes: i32, edges: i32) -> impl Graph {
    let mut g = StandardGraph::new();

    let prob_one_way = 0.2;

    for _ in 0..nodes {
        g.add_node(Node::default());
    }

    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..edges {
        let from = rng.gen_range(0..nodes);
        let to = rng.gen_range(0..nodes);
        let distance = rng.gen_range(1.0..10.0);
        let one_way = rng.gen_bool(prob_one_way);
        g.add_edge(Edge::new(from, to, distance, true, !one_way));
    }

    g
}

pub fn create_complex_graph() -> impl Graph {
    let mut g = StandardGraph::new();
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());

    g.add_edge(Edge::new(0, 1, 4.0, true, true));
    g.add_edge(Edge::new(0, 4, 2.0, true, true));
    g.add_edge(Edge::new(0, 6, 3.0, true, true));
    g.add_edge(Edge::new(1, 4, 1.0, true, true));
    g.add_edge(Edge::new(1, 3, 2.0, true, true));
    g.add_edge(Edge::new(1, 2, 3.0, true, true));
    g.add_edge(Edge::new(2, 6, 4.0, true, true));
    g.add_edge(Edge::new(5, 4, 4.0, true, true));
    g.add_edge(Edge::new(5, 6, 4.0, true, true));
    g.add_edge(Edge::new(3, 4, 5.0, true, true));
    g
}

pub fn create_ii_graph() -> impl Graph {
    let mut g = StandardGraph::new();
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());

    let edge1 = Edge::new(0, 1, 1.0, true, true);
    let edge2 = Edge::new(2, 3, 1.0, true, true);

    g.add_edge(edge1);
    g.add_edge(edge2);
    g
}

pub fn create_k3_graph() -> impl Graph {
    let mut g = StandardGraph::new();
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());

    let edge1 = Edge::new(0, 1, 1.0, true, true);
    let edge2 = Edge::new(0, 2, 1.0, true, true);
    let edge3 = Edge::new(1, 2, 1.0, true, true);

    g.add_edge(edge1);
    g.add_edge(edge2);
    g.add_edge(edge3);

    g
}

pub fn create_square_graph() -> impl Graph {
    let mut g = StandardGraph::new();
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());

    let edge1 = Edge::new(0, 1, 1.0, true, true);
    let edge2 = Edge::new(1, 2, 1.0, true, true);
    let edge3 = Edge::new(2, 3, 1.0, true, true);
    let edge4 = Edge::new(3, 0, 1.0, true, true);

    g.add_edge(edge1);
    g.add_edge(edge2);
    g.add_edge(edge3);
    g.add_edge(edge4);

    g
}
