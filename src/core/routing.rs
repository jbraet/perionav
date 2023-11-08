pub mod bidirdijkstra;
pub mod dijkstra;
pub mod dijkstra2;
pub mod heapentry;
pub mod heapentry2;
pub mod options;

pub use super::path::Path;

#[non_exhaustive]
pub struct RoutingResult {
    pub distance: f64,
    pub weight: f64,
    pub paths: Vec<Path>, //can be empty if path is not requested
}