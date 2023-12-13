use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use perionav::core::components::options::AlgorithmType as ComponentsAlgorithmType;
use perionav::core::components::options::ComponentsAlgorithmOptions;
use perionav::core::routing::options::{AlgorithmType, RoutingAlgorithmOptions, WeightType};
use perionav::core::Graph;
use perionav::reader::osm_reader::OsmReader;
use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let result = OsmReader::new("./data/flanders.osm.pbf");
    let graph_reader = match result {
        Ok(graph_reader) => graph_reader,
        Err(e) => panic!("something went wrong while opening the osm file: {}", e),
    };

    let result = graph_reader.read_graph();
    let mut g = match result {
        Ok(g) => g,
        Err(e) => panic!("something went wrong while reading the osm file: {}", e),
    };

    //TODO replace with benchmark
    println!("created graph in {} ms: nr edges={} & nr nodes={}", now.elapsed().as_millis(), g.get_nr_edges(), g.get_nr_nodes());
    let now = Instant::now();
    let opts = ComponentsAlgorithmOptions::new(ComponentsAlgorithmType::PATHBASED);
    let result = g.get_strongly_connected_subgraphs(&opts);
    println!("got {} components in {} ms", result.len(), now.elapsed().as_millis());

    let now = Instant::now();
    let opts2 = ComponentsAlgorithmOptions::new(ComponentsAlgorithmType::KOSARAJU);
    let result2 = g.get_strongly_connected_subgraphs(&opts2);
    println!("got {} components in {} ms", result2.len(), now.elapsed().as_millis());

    let now = Instant::now();
    let opts3 = ComponentsAlgorithmOptions::new(ComponentsAlgorithmType::TARJAN);
    let result3 = g.get_strongly_connected_subgraphs(&opts3);
    println!("got {} components in {} ms", result3.len(), now.elapsed().as_millis());

    let mut max_size = 0;
    let mut max_component = HashSet::new();
    for component in result {
        //used for some debugging
        if component.len() > 20 && component.len() < 5000 {
            println!("component has size {}", component.len());
            /*let visualisation = g.visualise_sub_graph(&component);
            println!("{}",visualisation);*/
        }

        if component.len() > max_size {
            max_size = component.len();
            max_component = component;
        }
    }

    println!("graph shrinking from {} nodes to {} nodes", g.get_nr_nodes(), max_component.len());
    let now = Instant::now();
    g.keep_nodes(&max_component);
    println!("filtered graph in {} ms. current nr edges: {}", now.elapsed().as_millis(), g.get_nr_edges());

    let now = Instant::now();
    let kdtree = g.create_kd_tree();
    println!("created kdtree in {} ms", now.elapsed().as_millis());

    let (from_lat, from_lon) = (51.046527, 3.719028);
    let (to_lat, to_lon) = (51.028482, 3.639622);

    let now = Instant::now();
    let from_node = find_closest_node(&kdtree, from_lat, from_lon);
    let to_node = find_closest_node(&kdtree, to_lat, to_lon);
    println!("routing from node {} to node {}", from_node, to_node);
    let opts = RoutingAlgorithmOptions::new(true, AlgorithmType::BIDIRDIJKSTRA, WeightType::DISTANCE);
    let result = g.route(&opts, from_node, to_node);
    if let Some(routing_result) = result {
        if !routing_result.paths.is_empty() {
            let nodes = routing_result.paths[0].get_wkt(&g);
            println!("result: {} in {} ms", nodes, now.elapsed().as_millis());
        } else {
            println!("no path found")
        }
    } else {
        println!("no route found")
    }

    //TODO create api
}

//TODO move this to somewhere more sensical
fn find_closest_node(kdtree: &KdTree<f64, i32, [f64; 2]>, lat: f64, lon: f64) -> i32 {
    let kd_nodes = kdtree.nearest(&[lat, lon], 1, &squared_euclidean).unwrap();
    *kd_nodes[0].1
}
