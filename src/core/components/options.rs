use std::collections::HashSet;

use crate::core::Graph;

use super::{
    kosaraju::KosarajuComponentsAlgorithm, pathbased::PathBasedComponentsAlgorithm, tarjan::TarjanComponentsAlgorithm,
    tarjan2::TarjanComponentsAlgorithm2,
};

#[non_exhaustive]
pub enum AlgorithmType {
    PATHBASED,
    TARJAN,
    TARJAN2, //different implementation of the same algorithm
    KOSARAJU,
}

/// Calculate strongly connected subcomponents of a graph
pub trait ComponentsAlgorithm<G: Graph> {
    fn get_components(&self, graph: &G) -> Vec<HashSet<usize>>;
}

pub struct ComponentsAlgorithmOptions<G: Graph> {
    pub components_algorithm: Box<dyn ComponentsAlgorithm<G>>,
}

impl<G: Graph> ComponentsAlgorithmOptions<G> {
    pub fn new(algorithm_type: AlgorithmType) -> Self {
        let components_algorithm = create_components_algorithm(&algorithm_type);
        ComponentsAlgorithmOptions { components_algorithm }
    }
}

pub fn create_components_algorithm<G: Graph>(algorithm_type: &AlgorithmType) -> Box<dyn ComponentsAlgorithm<G>> {
    match algorithm_type {
        AlgorithmType::PATHBASED => Box::new(PathBasedComponentsAlgorithm::new()),
        AlgorithmType::TARJAN => Box::new(TarjanComponentsAlgorithm {}),
        AlgorithmType::TARJAN2 => Box::new(TarjanComponentsAlgorithm2 {}),
        AlgorithmType::KOSARAJU => Box::new(KosarajuComponentsAlgorithm {}),
    }
}
