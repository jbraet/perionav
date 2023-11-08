use std::collections::HashSet;

use crate::core::Graph;

use super::{tarjan::TarjanComponentsAlgorithm, pathbased::PathBasedComponentsAlgorithm, kosaraju::KosarajuComponentsAlgorithm};

#[non_exhaustive]
pub enum AlgorithmType {
    PATHBASED,
    TARJAN,
    KOSARAJU,
}

/// Calculate strongly connected subcomponents of a graph
pub trait ComponentsAlgorithm<G:Graph> {
    fn get_components(&self, graph: &G) -> Vec<HashSet<i32>>;
}

pub struct ComponentsAlgorithmOptions<G:Graph> {
    pub components_algorithm: Box<dyn ComponentsAlgorithm<G>>,
}

impl<G:Graph> ComponentsAlgorithmOptions<G> {
    pub fn new(algorithm_type: AlgorithmType) -> Self {
        let components_algorithm = create_components_algorithm(&algorithm_type);
        ComponentsAlgorithmOptions{
            components_algorithm,
        }
    }
}

pub fn create_components_algorithm<G:Graph>(algorithm_type: &AlgorithmType) -> Box<dyn ComponentsAlgorithm<G>> {
   match algorithm_type {
        AlgorithmType::PATHBASED => Box::new(PathBasedComponentsAlgorithm{}),
        AlgorithmType::TARJAN => Box::new(TarjanComponentsAlgorithm{}),
        AlgorithmType::KOSARAJU => Box::new(KosarajuComponentsAlgorithm{}),
    }
}