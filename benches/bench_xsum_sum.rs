mod common;
use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use xsum::{XsumLarge, XsumSmall};

use crate::common::DATA_MAP_F64;

fn xsumsmall_add_list_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("xsum");
    for (size, array) in DATA_MAP_F64.iter() {
        group.bench_with_input(
            BenchmarkId::new("xsumsmall add_list", size),
            *array,
            |bench, arr| {
                bench.iter(|| {
                    let mut xsumsmall = XsumSmall::new();
                    xsumsmall.add_list(black_box(arr));
                    xsumsmall.sum();
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("xsumsmall add", size),
            *array,
            |bench, arr| {
                bench.iter(|| {
                    let mut xsumsmall = XsumSmall::new();
                    for v in arr {
                        xsumsmall.add(black_box(*v));
                    }
                    xsumsmall.sum();
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("xsumlarge add_list", size),
            *array,
            |bench, arr| {
                bench.iter(|| {
                    let mut xsumlarge = XsumLarge::new();
                    xsumlarge.add_list(black_box(arr));
                    xsumlarge.sum();
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("xsumlarge add", size),
            *array,
            |bench, arr| {
                bench.iter(|| {
                    let mut xsumlarge = XsumLarge::new();
                    for v in arr {
                        xsumlarge.add(black_box(*v));
                    }
                    xsumlarge.sum();
                })
            },
        );
    }
    group.finish();
}

criterion_group!(benches, xsumsmall_add_list_bench);
criterion_main!(benches);
