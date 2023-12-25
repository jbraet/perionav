use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use perionav::core::components::options::AlgorithmType as ComponentsAlgorithmType;
use perionav::core::components::options::ComponentsAlgorithmOptions;
use perionav::core::routing::options::{AlgorithmType, RoutingAlgorithmOptions, WeightType};
use perionav::core::routing::Path;
use perionav::core::routing::RoutingResult;
use perionav::core::Graph;
use perionav::reader::osm_reader::OsmReader;
use std::collections::HashSet;
use std::time::Instant;

pub struct Router<G: Graph> {
    graph: G,
    kdtree: KdTree<f64, usize, [f64; 2]>,
}

pub fn new_router(file_name: &str) -> Router<impl Graph> {
    let now = Instant::now();

    //TODO extract create graph logic ? also allow to use example graphs (the ones used for testing or so)
    let result = OsmReader::new(file_name);
    let graph_reader = match result {
        Ok(graph_reader) => graph_reader,
        Err(e) => panic!("something went wrong while opening the osm file: {}", e),
    };

    let result = graph_reader.read_graph();
    let mut graph = match result {
        Ok(graph) => graph,
        Err(e) => panic!("something went wrong while reading the osm file: {}", e),
    };
    println!("created graph in {} ms: nr edges={} & nr nodes={}", now.elapsed().as_millis(), graph.get_nr_edges(), graph.get_nr_nodes());

    let now = Instant::now();
    let opts = ComponentsAlgorithmOptions::new(ComponentsAlgorithmType::PATHBASED);
    let result = graph.get_strongly_connected_subgraphs(&opts);
    println!("got {} components in {} ms", result.len(), now.elapsed().as_millis());

    let mut max_size = 0;
    let mut max_component = HashSet::new();
    for component in result {
        //used for some debugging
        if component.len() > 20 && component.len() < 5000 {
            println!("component has size {}", component.len());
            /*let visualisation = g.visualise_sub_graph(&component);
            println!("{}", visualisation);*/
        }

        if component.len() > max_size {
            max_size = component.len();
            max_component = component;
        }
    }

    println!("graph shrinking from {} nodes to {} nodes", graph.get_nr_nodes(), max_component.len());
    let now = Instant::now();
    graph.keep_nodes(&max_component);
    println!("filtered graph in {} ms. current nr edges: {}", now.elapsed().as_millis(), graph.get_nr_edges());

    let now = Instant::now();
    let kdtree = graph.create_kd_tree();
    println!("created kdtree in {} ms", now.elapsed().as_millis());

    Router { graph, kdtree }
}

impl<G: Graph> Router<G> {
    pub fn route(&self, (from_lat, from_lon): (f64, f64), (to_lat, to_lon): (f64, f64)) -> Option<RoutingResult> {
        let from_node = self.find_closest_node(from_lat, from_lon);
        let to_node = self.find_closest_node(to_lat, to_lon);
        let opts = RoutingAlgorithmOptions::new(true, AlgorithmType::BIDIRDIJKSTRA, WeightType::DISTANCE);
        self.graph.route(&opts, from_node, to_node)
    }

    pub fn get_wkt(&self, path: &Path) -> String {
        path.get_wkt(&self.graph)
    }

    fn find_closest_node(&self, lat: f64, lon: f64) -> usize {
        let kd_nodes = self.kdtree.nearest(&[lat, lon], 1, &squared_euclidean).unwrap();
        *kd_nodes[0].1
    }
}
