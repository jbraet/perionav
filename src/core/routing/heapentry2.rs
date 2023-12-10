use crate::core::edge::DirectedVehicleSpecificEdgeInformation;
use ordered_float::NotNan;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use crate::core::edgeinformation::EdgeInformation;
// this is a second implementation of HeapEntry that keeps track if an entry is deleted with a simple bool
// this allows a different implementation of the dijkstra algorithm
// however according to benchmarks this is actually slower
pub struct HeapEntry {
    pub key: NotNan<f64>,
    pub value: i32,
    pub parent: Option<Rc<RefCell<HeapEntry>>>,
    pub edge: Option<Rc<EdgeInformation>>, //only relevant when parent isn’t None
    pub deleted: bool,
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

impl HeapEntry {
    /// key must be nonNaN
    pub fn new(key: f64, value: i32, directed_edge_info: Rc<DirectedVehicleSpecificEdgeInformation>, parent: Option<Rc<RefCell<HeapEntry>>>) -> Self {
        let edge_information = parent.as_ref().map(|p| {
            let base_node = p.borrow().value;

            Rc::new(EdgeInformation::new(base_node, value, directed_edge_info))
        });

        let notnan_key = NotNan::new(key).expect("given key is NAN");

        HeapEntry {
            key: notnan_key,
            value,
            parent,
            edge: edge_information,
            deleted: false,
        }
    }

    pub fn new_without_parent(key: f64, value: i32) -> Self {
        let notnan_key = NotNan::new(key).expect("given key is NAN");

        HeapEntry {
            key: notnan_key,
            value,
            parent: None,
            edge: None,
            deleted: false,
        }
    }

    pub fn get_path(&self) -> Vec<Rc<EdgeInformation>> {
        let mut ret = vec![];

        let mut curr = match &self.parent {
            None => return vec![],
            Some(p) => {
                ret.push(Rc::clone(self.edge.as_ref().unwrap()));
                Rc::clone(p)
            }
        };

        while curr.borrow().parent.is_some() {
            ret.push(Rc::clone(curr.borrow().edge.as_ref().unwrap()));
            let tmp = Rc::clone(curr.borrow().parent.as_ref().unwrap());
            curr = tmp;
        }

        ret.reverse();

        ret
    }
}
