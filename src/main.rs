use perionav::core::Graph;
use perionav::core::routing::options::{AlgorithmType, RoutingAlgorithmOptions, WeightType};
use perionav::reader::osm_reader::OsmReader;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let result = OsmReader::new("./data/gent2.osm.pbf");
    let graph_reader = match result {
        Ok(graph_reader) => graph_reader,
        Err(e) => panic!("something went wrong while opening the osm file: {}",e)
    };

    let result = graph_reader.read_graph();
    let g = match result {
        Ok(g) => g,
        Err(e) => panic!("something went wrong while reading the osm file: {}",e)
    };
    
    println!("created graph in {} seconds: nr edges={} & nr nodes={}", now.elapsed().as_secs(), g.get_nr_edges(), g.get_nr_nodes());
    let now = Instant::now();

    let strongly_connected = g.is_strongly_connected();
    println!("strongly connected: {} in {} seconds",strongly_connected, now.elapsed().as_secs());

    let (from_lat, from_lon) = (51.046527, 3.719028);
    let (to_lat, to_lon) = (51.028482, 3.639622);
    let from_node = g.find_closest_node(from_lat, from_lon);
    let to_node = g.find_closest_node(to_lat, to_lon);

    let opts = RoutingAlgorithmOptions::new(true, AlgorithmType::BIDIRDIJKSTRA,WeightType::DISTANCE);
    let result = g.route(&opts, from_node, to_node);
    if let Some(routing_result) = result {
        if !routing_result.paths.is_empty(){
            let nodes = routing_result.paths[0].get_wkt();
            println!("result: {}",nodes);
        } else {
            println!("no path found")
        }
    } else {
        println!("no route found")
    }

    //TODO create api    
}