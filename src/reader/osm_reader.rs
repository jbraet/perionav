use std::collections::HashMap;

use crate::core::{Node,Edge,StandardGraph,Graph};
use super::vehicle_permissions::*;

use osmpbf::{ElementReader, Element};

pub fn read_graph_from_osm(file: &str) -> Result<impl Graph,osmpbf::Error> {
    let reader = ElementReader::from_path(file)?;

    let mut g = StandardGraph::new();

    let mut nodes_map = HashMap::new(); //map from osm id to node id in graph
    let mut curr_node_index = 0; //the index inside the graph

    //TODO right now we make no distinction between tower nodes and normal nodes
    //tower nodes are crossing points of different ways while normal nodes are just there for some shape, traffic signs, traffic crossings, barriers, ...
    //implementing this should bring a decent performance gain since there will be less nodes to visit if normal nodes are ignored


    // ways always come after nodes
    reader.for_each(|element| {
        match element {
            Element::Way(way) => {

                let tags_map = convert_tags_to_map(way.tags());
                let (car_fwd, car_bwd)= is_car_allowed(&tags_map);

                let mut last_node=-1;
                if car_fwd || car_bwd {
                    for r in way.refs() {
                        let curr_node = *nodes_map.get(&r).unwrap_or(&-1);

                        if last_node==-1 {
                            last_node=curr_node;
                            continue
                        }

                        if curr_node!=-1 {
                            let edge = Edge::new(last_node,curr_node,1.0,car_fwd,car_bwd); //TODO distance is temporary
                            g.add_edge(edge);

                            last_node=curr_node;
                        }
                    }
                }
                
            },
            Element::Node(_) => {}
            Element::DenseNode(node) => {
                g.add_node(Node::new()); //TODO also store original id in the nodes
                nodes_map.insert(node.id, curr_node_index);
                curr_node_index+=1;
            },
            Element::Relation(_) => {}
        }
    })?;

    Result::Ok(g)
}