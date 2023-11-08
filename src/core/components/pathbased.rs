use std::collections::HashSet;

use crate::core::Graph;

use super::options::ComponentsAlgorithm;

pub struct PathBasedComponentsAlgorithm {}

impl<G:Graph> ComponentsAlgorithm<G> for PathBasedComponentsAlgorithm {
    fn get_components(&self, _graph: &G) -> Vec<HashSet<i32>> {
        vec![] //TODO
    }
}