use std::collections::HashMap;

use crate::{core::{Node,Edge,StandardGraph,Graph}, reader::tags_map::convert_tags_to_map};
use super::vehicle_permissions::*;

use osmpbf::{ElementReader, Element};

use geoutils::Location;

enum NodeType {
    TowerNode, //end of a way or the middle of a way that connects to another way
    ShapeNode, //middle of a way that's just there for shape
}

pub struct OsmReader<'a> {
    file_name: &'a str,

    node_types: HashMap<i64,NodeType>, // from node ID to nodetype
    way_permissions: HashMap<i64, (bool,bool)> //from way id to 
}

impl<'a> OsmReader<'a> {
    pub fn new(file_name: &'a str) -> Result<Self,osmpbf::Error> {
        let mut reader = OsmReader{
            file_name,
            node_types: HashMap::new(),
            way_permissions: HashMap::new(),
        };

        reader.categorize_nodes()?;
        Ok(reader)
    }

    pub fn read_graph(&self) -> Result<impl Graph,osmpbf::Error> {
        let reader = ElementReader::from_path(self.file_name)?;

        let mut g = StandardGraph::new();
        let mut curr_node_index = 0; //the index inside the graph

        let mut nodes_map = HashMap::new();
        let mut nr_ways = 0;

        // ways always come after nodes
        reader.for_each(|element| {
            match element {
                Element::Way(way) => {
                    nr_ways+=1;

                    let (car_fwd, car_bwd) = self.way_permissions.get(&way.id()).unwrap_or(&(false, false));

                    if *car_fwd || *car_bwd {
                        let mut last_node=-1;
                        let mut last_location = Location::new(0,0);
                        let mut curr_location = Location::new(0,0);

                        for node_id in way.refs() {
                            let curr_node = *nodes_map.get(&node_id).unwrap_or(&-1);

                            let n=g.get_node(curr_node);
                            if let Some(n) = n {
                                curr_location=Location::new(n.lat, n.lon);
                            }

                            if last_node==-1 && self.node_types.get(&node_id).is_some_and(|x| matches!(x,NodeType::TowerNode)) {
                                last_node = curr_node;
                                last_location = curr_location;
                                
                                continue
                            }

                            if last_node!=-1 && curr_node!=-1 && self.node_types.get(&node_id).is_some_and(|x| matches!(x,NodeType::TowerNode)) {
                                let dist = last_location.distance_to(&curr_location).unwrap().meters();

                                let edge = Edge::new(dist,*car_fwd,*car_bwd);
                                g.add_edge(last_node, curr_node, edge);

                                last_node = curr_node;
                                last_location = curr_location;
                            }
                        }                        
                    }
                },
                Element::Node(_) => {}
                Element::DenseNode(node) => {
                    if self.node_types.get(&node.id).is_some_and(|x| matches!(x,NodeType::TowerNode)) {
                        g.add_node(Node::new(node.id, node.lat(), node.lon()));
                        nodes_map.insert(node.id, curr_node_index);
                        curr_node_index+=1;
                    }
                },
                Element::Relation(_) => {}
            }
        })?;

        println!("nr ways parsed: {}",nr_ways);

        Result::Ok(g)
    }

    pub fn categorize_nodes(&mut self) -> Result<(),osmpbf::Error> {
        let reader = ElementReader::from_path(self.file_name)?;
        
        let mut nr_useful_ways = 0;

        reader.for_each(|element| {
            match element {
                Element::Way(way) => {
                    let tags_map = convert_tags_to_map(way.tags());
                    let (car_fwd, car_bwd)= is_car_allowed(&tags_map); // in future we might want to support more than just car

                    self.way_permissions.insert(way.id(), (car_fwd, car_bwd));

                    if car_fwd || car_bwd {
                        nr_useful_ways+=1;
                        let mut first = true;
                        let mut last = -1;
                        for node_id in way.refs() {
                            if first{
                                self.node_types.insert(node_id,NodeType::TowerNode);
                                first = false;
                            }

                            self.node_types.entry(node_id).and_modify(|e| *e = NodeType::TowerNode).or_insert(NodeType::ShapeNode);
                            last = node_id;
                        }

                        if last!=-1 {
                            self.node_types.insert(last, NodeType::TowerNode);
                        }
                    }
                },
                Element::Node(_) => {}
                Element::DenseNode(_) => {},
                Element::Relation(_) => {}
            }
        })?;

        println!("nr useful ways: {}", nr_useful_ways);

        Ok(())
    }
}