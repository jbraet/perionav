use perionav::core::Graph;
use perionav::reader::osm_reader::read_graph_from_osm;

fn main() {
    let result =read_graph_from_osm("./data/belgium-latest.osm.pbf");
    let g = match result {
        Ok(g) => g,
        Err(e) => panic!("something went wrong while reading the osm file: {}",e)
    };
    
    println!("strongly connected: {}",g.is_strongly_connected());

    //TODO create api    
}