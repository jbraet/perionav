use perionav::core::Graph;
use perionav::reader::osm_reader::read_graph_from_osm;
use std::time::Instant;

fn main() {
    let mut now = Instant::now();

    let result =read_graph_from_osm("./data/belgium-latest.osm.pbf");
    let g = match result {
        Ok(g) => g,
        Err(e) => panic!("something went wrong while reading the osm file: {}",e)
    };
    
    println!("created graph in {} seconds: nr edges={} & nr nodes={}", now.elapsed().as_secs(), g.get_nr_edges(), g.get_nr_nodes());
    now = Instant::now();

    let strongly_connected = g.is_strongly_connected();
    println!("strongly connected: {} in {} seconds",strongly_connected, now.elapsed().as_secs());

    //TODO create api    
}