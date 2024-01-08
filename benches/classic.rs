use criterion::{criterion_group, criterion_main, Criterion};

use my_algorithm_lab::classic::swapping;

pub fn swapping_large(c: &mut Criterion) {
    let mut large = c.benchmark_group("swapping(large)");

    large.bench_function("naive", |b| {
        let mut x = i32::MAX;
        let mut y = i32::MAX - 1;
        b.iter(|| swapping::swap_variables_naively(&mut x, &mut y));
    });
    large.bench_function("xor", |b| {
        let mut x = i32::MAX;
        let mut y = i32::MAX - 1;
        b.iter(|| swapping::swap_variables_xor(&mut x, &mut y));
    });
    large.bench_function("std_mem", |b| {
        let mut x = i32::MAX;
        let mut y = i32::MAX - 1;
        b.iter(|| swapping::swap_by_std_mem_swap(&mut x, &mut y));
    });
}

pub fn swapping_small(c: &mut Criterion) {
    let mut small = c.benchmark_group("swapping(small)");
    small.bench_function("naive", |b| {
        let mut x = 1;
        let mut y = 0;
        b.iter(|| swapping::swap_variables_naively(&mut x, &mut y));
    });
    small.bench_function("xor", |b| {
        let mut x = 1;
        let mut y = 0;
        b.iter(|| swapping::swap_variables_xor(&mut x, &mut y));
    });
    small.bench_function("std_mem", |b| {
        let mut x = 1;
        let mut y = 0;
        b.iter(|| swapping::swap_by_std_mem_swap(&mut x, &mut y));
    });
}

criterion_group!(benches, swapping_large, swapping_small);
criterion_main!(benches);
