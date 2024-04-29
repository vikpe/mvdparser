use std::fs;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use mvdparser::matchdate;

// get .mvd files from
// https://github.com/vikpe/mvdparser/tree/main/tests/files
fn get_demo_data() -> Vec<u8> {
    fs::read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd").expect("unable to read demo")
}

fn lib_benchmark(c: &mut Criterion) {
    let data = get_demo_data();
    let mut group = c.benchmark_group("lib");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("ktxstats", |b| b.iter(|| mvdparser::ktxstats(&data)));

    group.bench_function("matchdate", |b| b.iter(|| matchdate::matchdate(&data)));
    group.bench_function("matchdate_string", |b| {
        b.iter(|| matchdate::matchdate_string(&data))
    });

    group.bench_function("serverinfo", |b| b.iter(|| mvdparser::serverinfo(&data)));
    group.bench_function("serverinfo_string", |b| {
        b.iter(|| mvdparser::serverinfo_string(&data))
    });

    group.bench_function("timestamp", |b| b.iter(|| mvdparser::timestamp(&data)));

    group.finish();
}

criterion_group!(benches, lib_benchmark);
criterion_main!(benches);
