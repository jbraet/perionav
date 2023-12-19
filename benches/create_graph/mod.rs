use perionav::core::edge::Edge;
use perionav::core::node::Node;
use perionav::core::Graph;
use perionav::core::StandardGraph;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn create_random_graph(nodes: usize, edges: usize) -> impl Graph {
    let mut g = StandardGraph::new(100);

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
        g.add_edge(from, to, Edge::new(distance, true, !one_way));
    }

    g
}

pub fn create_complex_graph() -> impl Graph {
    let mut g = StandardGraph::new(100);
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());

    g.add_edge(0, 1, Edge::new(4.0, true, true));
    g.add_edge(0, 4, Edge::new(2.0, true, true));
    g.add_edge(0, 6, Edge::new(3.0, true, true));
    g.add_edge(1, 4, Edge::new(1.0, true, true));
    g.add_edge(1, 3, Edge::new(2.0, true, true));
    g.add_edge(1, 2, Edge::new(3.0, true, true));
    g.add_edge(2, 6, Edge::new(4.0, true, true));
    g.add_edge(5, 4, Edge::new(4.0, true, true));
    g.add_edge(5, 6, Edge::new(4.0, true, true));
    g.add_edge(3, 4, Edge::new(5.0, true, true));
    g
}

pub fn create_ii_graph() -> impl Graph {
    let mut g = StandardGraph::new(100);
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());

    g.add_edge(0, 1, Edge::new(1.0, true, true));
    g.add_edge(2, 3, Edge::new(1.0, true, true));
    g
}

pub fn create_k3_graph() -> impl Graph {
    let mut g = StandardGraph::new(100);
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());

    g.add_edge(0, 1, Edge::new(1.0, true, true));
    g.add_edge(0, 2, Edge::new(1.0, true, true));
    g.add_edge(1, 2, Edge::new(1.0, true, true));

    g
}

pub fn create_square_graph() -> impl Graph {
    let mut g = StandardGraph::new(100);
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());

    g.add_edge(0, 1, Edge::new(1.0, true, true));
    g.add_edge(1, 2, Edge::new(1.0, true, true));
    g.add_edge(2, 3, Edge::new(1.0, true, true));
    g.add_edge(3, 0, Edge::new(1.0, true, true));

    g
}
