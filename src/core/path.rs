use std::rc::Rc;

use super::edgeinformation::EdgeInformation;

//each edge must be linked to the next edge
pub struct Path {
    edges: Vec<Rc<EdgeInformation>>,
} 

impl Path {
    pub fn new(edges: Vec<Rc<EdgeInformation>>) -> Self {
        Path::check_edges_valid(&edges, None);

        Path {
            edges,
        }
    }

    pub fn add_edges(&mut self, edges: Vec<Rc<EdgeInformation>>) {
        let last_node = self.edges.last().map(|e| {
            e.adj_node
        });

        Path::check_edges_valid(&edges, last_node);

        self.edges.extend(edges);
    }

    fn check_edges_valid(edges: &Vec<Rc<EdgeInformation>>, mut last_node: Option<i32>) {
        for edge in edges {
            if let Some(last_node) = last_node {
                let base_node = edge.get_base_node();
                if last_node !=  base_node{
                    panic!("edges aren't connected: last node {} doesnt match current node {}", last_node, base_node)
                }
            }

            last_node = Some(edge.adj_node)
        }
    }

    pub fn get_nodes(&self) -> Vec<i32> {
        let mut ret = vec![];
        let mut start = true;
        for edge_info in &self.edges {
            if start {
                let base_node = edge_info.edge.get_adj_node(edge_info.adj_node);
                ret.push(base_node);
                start = false;
            }

            ret.push(edge_info.adj_node);
        }

        ret
    }
}