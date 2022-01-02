pub mod custom_hashing;
pub mod sort;

#[cfg(test)]
mod tests {
    use crate::leetcode::common::test_utils::from_file;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use std::path::PathBuf;

    use super::*;
    use serde::Deserialize;
    use test_case::test_case;

    #[derive(Deserialize)]
    struct TestCase {
        input: Vec<String>,
        answer: Vec<Vec<String>>,
    }

    #[test_case("example1.ron")]
    #[test_case("case1.ron")]
    fn test_solution(str: &str) {
        let TestCase { input, answer } =
            from_file(PathBuf::new().join("src/leetcode/algorithm_49").join(str));
        assert_eq!(
            HashSet::<Vec<String>>::from_iter(answer.into_iter()),
            HashSet::from_iter(custom_hashing::Solution::group_anagrams(input).into_iter())
        );
    }

    #[test_case("example1.ron")]
    #[test_case("case1.ron")]
    fn test_sort_solution(str: &str) {
        let TestCase { input, answer } =
            from_file(PathBuf::new().join("src/leetcode/algorithm_49").join(str));
        assert_eq!(
            HashSet::<Vec<String>>::from_iter(answer.into_iter()),
            HashSet::from_iter(sort::Solution::group_anagrams(input).into_iter())
        );
    }
}
