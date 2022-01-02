pub mod hash_btree;
pub mod sort;

#[cfg(test)]
mod tests {
    use crate::leetcode::common::test_utils::from_file;
    use std::collections::{BTreeSet, HashSet};
    use std::iter::FromIterator;
    use std::path::PathBuf;

    use super::*;
    use serde::Deserialize;
    use test::Bencher;
    use test_case::test_case;

    #[derive(Deserialize)]
    struct TestCase {
        input: Vec<String>,
        answer: Vec<Vec<String>>,
    }

    #[test_case("example1.ron")]
    #[test_case("case1.ron")]
    #[test_case("case2.ron")]
    #[test_case("case3.ron")]
    #[test_case("case4.ron")]
    fn test_sort_solution(str: &str) {
        let TestCase { input, answer } =
            from_file(PathBuf::new().join("src/leetcode/algorithm_49").join(str));
        assert_eq!(
            HashSet::<BTreeSet<String>>::from_iter(
                answer
                    .into_iter()
                    .map(|v| BTreeSet::from_iter(v.into_iter()))
            ),
            HashSet::<BTreeSet<String>>::from_iter(
                sort::Solution::group_anagrams(input)
                    .into_iter()
                    .map(|v| BTreeSet::from_iter(v.into_iter()))
            )
        );
    }

    #[test_case("example1.ron")]
    #[test_case("case1.ron")]
    #[test_case("case2.ron")]
    #[test_case("case3.ron")]
    #[test_case("case4.ron")]
    fn test_hash_btree_solution(str: &str) {
        let TestCase { input, answer } =
            from_file(PathBuf::new().join("src/leetcode/algorithm_49").join(str));
        assert_eq!(
            HashSet::<BTreeSet<String>>::from_iter(
                answer
                    .into_iter()
                    .map(|v| BTreeSet::from_iter(v.into_iter()))
            ),
            HashSet::<BTreeSet<String>>::from_iter(
                hash_btree::Solution::group_anagrams(input)
                    .into_iter()
                    .map(|v| BTreeSet::from_iter(v.into_iter()))
            )
        );
    }

    const BENCH_INPUT: &[&str] = &[
        "abc",
        "cba",
        "aaa",
        "aaab",
        "cccb",
        "aaaaaaaaaaaaaaaaaaab",
        "aaaaaaaaabaaaaaaaaa",
    ];
    #[bench]
    fn bench_sort_solution(b: &mut Bencher) {
        let input: Vec<String> = BENCH_INPUT.into_iter().map(|s| s.to_string()).collect();
        b.iter(move || sort::Solution::group_anagrams(input.clone()))
    }

    #[bench]
    fn bench_hash_btree_solution(b: &mut Bencher) {
        let input: Vec<String> = BENCH_INPUT.into_iter().map(|s| s.to_string()).collect();
        b.iter(move || hash_btree::Solution::group_anagrams(input.clone()))
    }
}
