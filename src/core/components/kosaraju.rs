use std::collections::HashSet;

use crate::core::Graph;

use super::options::ComponentsAlgorithm;

pub struct KosarajuComponentsAlgorithm {}

impl<G:Graph> ComponentsAlgorithm<G> for KosarajuComponentsAlgorithm {
    fn get_components(&self, _graph: &G) -> Vec<HashSet<i32>> {
        vec![] //TODO
    }
}