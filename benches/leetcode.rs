use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use my_algorithm_lab::leetcode::{algorithm_48, algorithm_49, algorithm_56, algorithm_76};

pub fn leetcode_48(c: &mut Criterion) {
    fn create_matrix(size: usize) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; size]; size];
        for row in matrix.iter_mut() {
            for (column, value) in row.iter_mut().enumerate() {
                *value = (column % 2) as i32;
            }
        }
        matrix
    }
    let mut algorithm = c.benchmark_group("leetcode 48");
    algorithm.bench_function("mirror", |b| {
        let mut matrix = create_matrix(100);
        b.iter(move || algorithm_48::mirror::Solution::rotate(&mut matrix));
    });
    algorithm.bench_function("complicate_reverse_assign", |b| {
        let mut matrix = create_matrix(100);
        b.iter(move || algorithm_48::complicate_reverse_assign::Solution::rotate(&mut matrix));
    });
    algorithm.bench_function("complicate", |b| {
        let mut matrix = create_matrix(100);
        b.iter(move || algorithm_48::complicate::Solution::rotate(&mut matrix));
    });
    algorithm.bench_function("complicate_swap", |b| {
        let mut matrix = create_matrix(100);
        b.iter(move || algorithm_48::complicate_swap::Solution::rotate(&mut matrix));
    });
}

pub fn leetcode_49(c: &mut Criterion) {
    const BENCH_INPUT: &[&str] = &[
        "abc",
        "cba",
        "aaa",
        "aaab",
        "cccb",
        "aaaaaaaaaaaaaaaaaaab",
        "aaaaaaaaabaaaaaaaaa",
    ];
    let input: Vec<String> = BENCH_INPUT.iter().map(|s| s.to_string()).collect();
    let mut algorithm = c.benchmark_group("leetcode 49");
    algorithm.bench_function("sort", |b| {
        b.iter_batched(
            || input.clone(),
            algorithm_49::sort::Solution::group_anagrams,
            BatchSize::SmallInput,
        )
    });
    algorithm.bench_function("hash_btree", |b| {
        b.iter_batched(
            || input.clone(),
            algorithm_49::hash_btree::Solution::group_anagrams,
            BatchSize::SmallInput,
        )
    });
}

pub fn leetcode_56(c: &mut Criterion) {
    use my_algorithm_lab::leetcode::common::test_utils::from_file;
    use serde::Deserialize;
    #[derive(Deserialize)]
    struct TestCase {
        input: Vec<Vec<i32>>,
    }
    fn load_input() -> Vec<Vec<i32>> {
        let TestCase { input } = from_file("src/leetcode/algorithm_56/example1.ron");
        input
    }
    let mut algorithm = c.benchmark_group("leetcode 56");
    algorithm.bench_function("normal", |b| {
        b.iter_batched(
            load_input,
            algorithm_56::normal::Solution::merge,
            BatchSize::SmallInput,
        );
    });
    algorithm.bench_function("convergence", |b| {
        b.iter_batched(
            load_input,
            algorithm_56::convergence::Solution::merge,
            BatchSize::SmallInput,
        );
    });
}

pub fn leetcode_76(c: &mut Criterion) {
    let mut algorithm = c.benchmark_group("leetcode 76");
    algorithm.bench_function("sliding window", |b| {
        b.iter(|| algorithm_76::sliding_window::Solution::min_window("ADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANCADOBECODEBANC".to_string(), "ABC". to_string()));
    });
}

criterion_group!(benches, leetcode_48, leetcode_49, leetcode_76);
criterion_main!(benches);
