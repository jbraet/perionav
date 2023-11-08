use perionav::core::create_graph;

fn main() {
    let g = create_graph();
    println!("graph connected: {}", g.is_strongly_connected());

    //TODO read graph from pbf file

    //TODO create api
}
