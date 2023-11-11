use perionav::core::graph::Graph;
use perionav::core::edge::Edge;
use perionav::core::standardgraph::StandardGraph;
use perionav::core::node::Node;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn create_random_graph(nodes: i32, edges: i32) -> impl Graph {
    let mut g = StandardGraph::new(100);

    let prob_one_way = 1.0;

    for _ in 0..nodes {
        g.add_node(Node::default());
    }

    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..edges {
        let mut from = rng.gen_range(0..nodes);
        let mut to = rng.gen_range(0..nodes);
        while from==to {
            from = rng.gen_range(0..nodes);
            to = rng.gen_range(0..nodes);
        }
        let distance = rng.gen_range(3.0..5.0);
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

pub fn create_small_sub_components_graph() -> impl Graph {
    let mut g = StandardGraph::new(100);
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());

    g.add_edge(0, 1, Edge::new(1.0, true, false));
    g.add_edge(1, 2, Edge::new(1.0, true, false));
    g.add_edge(2, 0, Edge::new(1.0, true, false));

    g.add_edge(3, 1, Edge::new(1.0, true, false));
    g.add_edge(3, 2, Edge::new(1.0, true, false));
    g.add_edge(3, 4, Edge::new(1.0, true, true));

    g
}

pub fn create_sub_components_graph() -> impl Graph {
    let mut g = StandardGraph::new(100);
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());

    g.add_edge(0, 1, Edge::new(1.0, true, false));
    g.add_edge(1, 2, Edge::new(1.0, true, false));
    g.add_edge(2, 0, Edge::new(1.0, true, false));

    g.add_edge(3, 1, Edge::new(1.0, true, false));
    g.add_edge(3, 2, Edge::new(1.0, true, false));
    g.add_edge(3, 4, Edge::new(1.0, true, true));

    g.add_edge(4, 5, Edge::new(1.0, true, false));
    g.add_edge(5, 2, Edge::new(1.0, true, false));
    g.add_edge(5, 6, Edge::new(1.0, true, true));

    g.add_edge(7, 4, Edge::new(1.0, true, false));
    g.add_edge(7, 6, Edge::new(1.0, true, false));

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

pub fn create_connected_ii_graph() -> impl Graph {
    let mut g = StandardGraph::new(100);
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());

    g.add_edge(0, 1, Edge::new(1.0, true, true));
    g.add_edge(2, 3, Edge::new(1.0, true, true));
    g.add_edge(0, 2, Edge::new(1.0, true, false));

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

pub fn create_line_graph() -> impl Graph {
    let mut g = StandardGraph::new(100);
    g.add_node(Node::default());
    g.add_node(Node::default());

    g.add_edge(0, 1, Edge::new(1.0, true, true));

    g
}