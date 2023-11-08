use crate::core::edge::Edge;
use ordered_float::NotNan;
use std::cmp::Ordering;
use std::rc::Rc;

use crate::core::edgeinformation::EdgeInformation;

pub struct HeapEntry {
    pub key: NotNan<f64>, // distance from start/end
    pub value: i32, // node
    pub parent: Option<Rc<HeapEntry>>,
    pub edge: Option<Rc<EdgeInformation>>, //only relevant when parent isn't None
}

impl Eq for HeapEntry {}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.key.cmp(&self.key) //reversed
    }
}

pub fn create_edge_information(edge: Option<Rc<Edge>>, base_node: i32, adj_node: i32, reverse: bool) -> Option<Rc<EdgeInformation>> {
    edge.map(|e| {
        Rc::new(EdgeInformation {
            edge: e,
            adj_node: if !reverse{adj_node} else {base_node},
        })
    })
}

impl HeapEntry {
    /// key must be nonNaN
    pub fn new(
        key: f64,
        value: i32,
        edge_information: Option<Rc<EdgeInformation>>,
        parent: Option<Rc<HeapEntry>>,
    ) -> Self {
        let notnan_key = NotNan::new(key).expect("given key is NAN");

        HeapEntry {
            key: notnan_key,
            value,
            parent,
            edge: edge_information,
        }
    }

    pub fn get_path(&self, reverse:bool) -> Vec<Rc<EdgeInformation>> {
        let mut ret = vec![];

        let mut curr = match &self.parent {
            None => return vec![],
            Some(p) => {
                ret.push(Rc::clone(self.edge.as_ref().unwrap()));
                Rc::clone(p)
            }
        };

        while curr.parent.is_some() {
            ret.push(Rc::clone(curr.edge.as_ref().unwrap()));
            curr = Rc::clone(curr.parent.as_ref().unwrap());
        }

        if reverse {
            ret.reverse();
        }

        ret
    }
}