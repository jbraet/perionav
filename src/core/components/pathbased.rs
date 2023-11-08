use std::collections::HashSet;

use crate::core::Graph;

use super::options::ComponentsAlgorithm;

pub struct PathBasedComponentsAlgorithm {}

impl<G:Graph> ComponentsAlgorithm<G> for PathBasedComponentsAlgorithm {
    fn get_components(&self, _graph: &G, _start: i32, _end: i32) -> Vec<HashSet<i32>> {
        vec![] //TODO
    }
}