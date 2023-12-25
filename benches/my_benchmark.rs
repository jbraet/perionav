use criterion::{criterion_group, criterion_main, Criterion};
use perionav::core::components::options::{AlgorithmType as ComponentsAlgorithmType, ComponentsAlgorithmOptions};
use perionav::core::routing::options::{AlgorithmType, RoutingAlgorithmOptions, WeightType};
use perionav::core::Graph;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

mod create_graph;

fn routing_benchmark(c: &mut Criterion) {
    let nodes = 200000;
    let g = create_graph::create_random_graph(nodes, 4 * nodes);
    let opts = RoutingAlgorithmOptions::new(true, AlgorithmType::DIJKSTRA, WeightType::DISTANCE);
    let opts2 = RoutingAlgorithmOptions::new(true, AlgorithmType::DIJKSTRA2, WeightType::DISTANCE);
    let opts3 = RoutingAlgorithmOptions::new(true, AlgorithmType::BIDIRDIJKSTRA, WeightType::DISTANCE);

    let mut rng = StdRng::seed_from_u64(42);

    /*c.bench_function("dijkstra1", |b| {
        b.iter(|| {
            let from = rng.gen_range(0..nodes);
            let to = rng.gen_range(0..nodes);
            g.route(&opts2, from, to)
        })
    });*/

    let mut group = c.benchmark_group("Dijkstra");

    group.bench_function("dijkstra1", |b| {
        b.iter(|| {
            let from = rng.gen_range(0..nodes);
            let to = rng.gen_range(0..nodes);
            g.route(&opts, from, to)
        })
    });

    group.bench_function("dijkstra2", |b| {
        b.iter(|| {
            let from = rng.gen_range(0..nodes);
            let to = rng.gen_range(0..nodes);
            g.route(&opts2, from, to)
        })
    });

    group.bench_function("bidirdijkstra", |b| {
        b.iter(|| {
            let from = rng.gen_range(0..nodes);
            let to = rng.gen_range(0..nodes);
            g.route(&opts3, from, to)
        })
    });

    group.finish();
}

fn components_benchmark(c: &mut Criterion) {
    let g = create_graph::create_flanders_graph();

    let opts = ComponentsAlgorithmOptions::new(ComponentsAlgorithmType::PATHBASED);
    let opts2 = ComponentsAlgorithmOptions::new(ComponentsAlgorithmType::KOSARAJU);
    let opts3 = ComponentsAlgorithmOptions::new(ComponentsAlgorithmType::TARJAN);

    let mut group = c.benchmark_group("Components");

    group.bench_function("pathBased", |b| b.iter(|| g.get_strongly_connected_subgraphs(&opts)));

    group.bench_function("Kosaraju", |b| b.iter(|| g.get_strongly_connected_subgraphs(&opts2)));

    group.bench_function("Tarjan", |b| b.iter(|| g.get_strongly_connected_subgraphs(&opts3)));

    group.finish();
}

criterion_group!(benches, routing_benchmark, components_benchmark);
//criterion_group!(benches, components_benchmark); // only benchmark components
criterion_main!(benches);
