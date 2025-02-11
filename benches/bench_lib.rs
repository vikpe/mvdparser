use std::fs;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

// get .mvd files from
// https://github.com/vikpe/mvdparser/tree/main/tests/files
fn get_demo_data() -> Vec<u8> {
    fs::read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd").expect("unable to read demo")
}

fn lib_benchmark(c: &mut Criterion) {
    let data = get_demo_data();
    let mut group = c.benchmark_group("lib");
    group.throughput(Throughput::Bytes(data.len() as u64));

    let file = &mut fs::File::open("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")
        .expect("unable to open demo 1");

    group.bench_function("prints", |b| b.iter(|| mvdparser::prints(file)));

    group.bench_function("serverinfo", |b| b.iter(|| mvdparser::serverinfo(file)));

    let file2 = &mut fs::File::open("tests/files/4on4_-s-_vs_pol[dm2]20241118-2135.mvd")
        .expect("unable to open demo 2");

    group.bench_function("is_paused (not paused)", |b| {
        b.iter(|| mvdparser::is_paused(file));
    });
    group.bench_function("is_paused (paused)", |b| {
        b.iter(|| mvdparser::is_paused(file2));
    });
    group.bench_function("clientinfo", |b| b.iter(|| mvdparser::clientinfo(file)));

    group.bench_function("matchdate", |b| b.iter(|| mvdparser::matchdate(file)));

    group.bench_function("ktxstats_string", |b| {
        b.iter(|| mvdparser::ktxstats_string(file))
    });
    group.bench_function("ktxstats_v3", |b| b.iter(|| mvdparser::ktxstats_v3(file)));

    group.bench_function("validate", |b| b.iter(|| mvdparser::all::is_paused(file)));

    /*
    group.bench_function("is_valid", |b| b.iter(|| mvdparser::is_valid(&data)));
    group.bench_function("aborted", |b| b.iter(|| mvdparser::is_aborted(&data)));
    group.bench_function("players", |b| b.iter(|| mvdparser::all::players(&data)));
    group.bench_function("players_from_parsing", |b| {
        b.iter(|| mvdparser::all::players_from_parsing(&data))
    });
    group.bench_function("teams", |b| b.iter(|| mvdparser::all::teams(&data)));

    group.bench_function("ping", |b| {
        b.iter(|| mvdparser::all::ping_per_player_number(&data))
    });

    group.bench_function("frags", |b| {
        b.iter(|| mvdparser::all::frags_per_player_name(&data))
    });




    group.bench_function("clients", |b| b.iter(|| mvdparser::all::clients(&data)));




    */

    /*

    group.bench_function("serverinfo_string", |b| {
        b.iter(|| mvdparser::all::serverinfo_string(file))
    });*/

    // group.bench_function("timestamp", |b| b.iter(|| mvdparser::all::timestamp(file)));

    // group.bench_function("matchdate", |b| b.iter(|| mvdparser::all::matchdate(file)));
    /*group.bench_function("matchdate_string", |b| {
        b.iter(|| mvdparser::all::matchdate_string(file))
    });*/

    /*

    group.bench_function("countdown_duration", |b| {
        b.iter(|| mvdparser::all::countdown_duration(&data))
    });
    group.bench_function("demo_duration", |b| {
        b.iter(|| mvdparser::all::demo_duration(&data))
    });
    group.bench_function("match_duration", |b| {
        b.iter(|| mvdparser::all::match_duration(&data))
    });*/

    group.finish();
}

criterion_group!(benches, lib_benchmark);
criterion_main!(benches);
