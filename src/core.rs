pub mod edge;
pub mod standardgraph;
pub mod node;
pub mod routing;
pub mod routingoptions;
pub mod edgeinformation;
pub mod weight;
pub mod path;
pub mod graph;

pub use standardgraph::*;
pub use graph::*;

///Creates an example graph
pub fn create_graph() -> impl Graph {
    let mut g = StandardGraph::new();
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());
    g.add_node(Node::default());

    let edge1 = Edge::new(0, 1, 1.0, true, true);
    let edge2 = Edge::new(2, 3, 1.0, true, true);
    let edge3 = Edge::new(1, 2, 1.0, true, true);

    g.add_edge(edge1);
    g.add_edge(edge2);
    g.add_edge(edge3);

    g
}
