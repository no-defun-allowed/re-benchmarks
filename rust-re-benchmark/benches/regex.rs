use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use regex::RegexBuilder;
use hyperscan::prelude::*;
use std::convert::TryInto;

fn logging(c: &mut Criterion) {
    let log = include_str!("../../Xorg.0.log");
    let resolution = RegexBuilder::new("([0-9]+)x([0-9]+)").unicode(false).build().unwrap();
    let quote_resolution = RegexBuilder::new("\"([0-9]+)x([0-9]+)\"").unicode(false).build().unwrap();

    let mut log_group = c.benchmark_group("Xorg.0.log");
    log_group.throughput(Throughput::Bytes(log.len().try_into().unwrap()));
    log_group.bench_function("no quotes, no submatches",
                             |b| b.iter(|| resolution.find_iter(log).count()));
    log_group.bench_function("no quotes, submatches",
                             |b| b.iter(|| resolution.captures_iter(log).count()));
    log_group.bench_function("quotes, no submatches",
                             |b| b.iter(|| quote_resolution.find_iter(log).count()));
    log_group.bench_function("quotes, submatches",
                             |b| b.iter(|| quote_resolution.captures_iter(log).count()));
    log_group.finish();
}

fn hyperscan(c: &mut Criterion) {
    
    let log = include_str!("../../Xorg.0.log");
    let resolution = &pattern!{"([0-9]+)x([0-9]+)"; SOM_LEFTMOST}.build().unwrap();
    let resolution_scratch = resolution.alloc_scratch().unwrap();
    let quoted = &pattern!{"\"([0-9]+)x([0-9]+)\""; SOM_LEFTMOST}.build().unwrap();
    let quoted_scratch = quoted.alloc_scratch().unwrap();

    let scan = |b: &BlockDatabase, s: &Scratch|
               b.scan(log, s, |_, _, _, _| Matching::Continue).unwrap();
    
    let mut log_group = c.benchmark_group("Hyperscan");
    log_group.throughput(Throughput::Bytes(log.len().try_into().unwrap()));
    log_group.bench_function("no quotes",
                             |b| b.iter(|| scan(resolution, &resolution_scratch)));
    log_group.bench_function("quotes",
                             |b| b.iter(|| scan(quoted, &quoted_scratch)));
    log_group.finish();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    logging(c);
    hyperscan(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
