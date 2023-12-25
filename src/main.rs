use std::time::Instant;

mod router;

fn main() {
    let router = router::new_router("./data/flanders.osm.pbf");

    let (from_lat, from_lon) = (51.046527, 3.719028);
    let (to_lat, to_lon) = (51.028482, 3.639622);

    let now = Instant::now();
    let result = router.route((from_lat, from_lon), (to_lat, to_lon));

    if let Some(routing_result) = result {
        if !routing_result.paths.is_empty() {
            let nodes = router.get_wkt(&routing_result.paths[0]);
            println!("result: {} in {} ms", nodes, now.elapsed().as_millis());
        } else {
            println!("no path found")
        }
    } else {
        println!("no route found")
    }

    //TODO create api
}
