use std::rc::Rc;

use super::{edgeinformation::EdgeInformation, Graph};

//each edge must be linked to the next edge
pub struct Path {
    edges: Vec<Rc<EdgeInformation>>,
}

impl Path {
    //TODO does this need to be Rc's ?
    pub fn new(edges: Vec<Rc<EdgeInformation>>) -> Self {
        Path::check_edges_valid(&edges, None);

        Path { edges }
    }

    pub fn add_edge(&mut self, edge: Rc<EdgeInformation>) {
        let last_node = self.edges.last().map(|e| e.get_adj_node());

        if let Some(last_node) = last_node {
            let base_node = edge.get_base_node();
            if base_node != last_node {
                panic!("edges aren't connected: last node {} doesnt match current node {}", last_node, base_node)
            }
        }

        self.edges.push(edge);
    }

    pub fn add_edges(&mut self, edges: Vec<Rc<EdgeInformation>>) {
        let last_node = self.edges.last().map(|e| e.get_adj_node());

        Path::check_edges_valid(&edges, last_node);

        self.edges.extend(edges);
    }

    fn check_edges_valid(edges: &Vec<Rc<EdgeInformation>>, mut last_node: Option<usize>) {
        for edge in edges {
            if let Some(last_node) = last_node {
                let base_node = edge.get_base_node();
                if last_node != base_node {
                    panic!("edges aren't connected: last node {} doesnt match current node {}", last_node, base_node)
                }
            }

            last_node = Some(edge.get_adj_node())
        }
    }

    pub fn get_wkt(&self, graph: &impl Graph) -> String {
        let mut first = false;
        let res = self.edges.iter().fold(vec![], |mut acc, e| {
            if first {
                let base_node = e.get_base_node();
                let node = graph.get_node(base_node).unwrap();
                acc.push(format!("{:.6} {:.6}", node.lon, node.lat)); //WKT uses lon lat

                first = false;
            }

            let adj_node = e.get_adj_node();
            let node = graph.get_node(adj_node).unwrap();

            acc.push(format!("{:.6} {:.6}", node.lon, node.lat)); //WKT uses lon lat

            acc
        });

        format!("LINESTRING({})", res.join(","))
    }

    pub fn get_nodes(&self) -> Vec<usize> {
        let mut ret = vec![];
        let mut start = true;

        for edge_info in &self.edges {
            if start {
                let base_node = edge_info.get_base_node();
                ret.push(base_node);
                start = false;
            }

            ret.push(edge_info.get_adj_node());
        }

        ret
    }
}
