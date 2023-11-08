#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use perionav::core::routingoptions::{AlgorithmOptions, AlgorithmType, WeightType};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

mod create_graph;

fn criterion_benchmark(c: &mut Criterion) {
    let nodes = 200000;
    let g = create_graph::create_random_graph(nodes, 4 * nodes);
    let opts = AlgorithmOptions::new(true,AlgorithmType::DIJKSTRA,WeightType::DISTANCE);
    let opts2 = AlgorithmOptions::new(true,AlgorithmType::DIJKSTRA2,WeightType::DISTANCE);
    let opts3 = AlgorithmOptions::new(true, AlgorithmType::BIDIRDIJKSTRA,WeightType::DISTANCE);

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

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
