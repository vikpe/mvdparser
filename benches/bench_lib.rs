use std::fs;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

fn get_demo_data() -> Vec<u8> {
    fs::read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd").expect("unable to read file")
}

fn lib_benchmark(c: &mut Criterion) {
    let data = get_demo_data();
    let mut group = c.benchmark_group("lib");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("matchdate", |b| b.iter(|| mvdparser::matchdate(&data)));    
    group.finish();
}

criterion_group!(benches, lib_benchmark);
criterion_main!(benches);
