pub mod complicate;
pub mod complicate_reverse_assign;
pub mod complicate_swap;
pub mod mirror;
pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::leetcode::common::test_utils::from_file;

    use super::*;
    use serde::Deserialize;
    use std::path::PathBuf;
    use test_case::test_case;

    #[derive(Deserialize)]
    struct TestCase {
        input: Vec<Vec<i32>>,
        answer: Vec<Vec<i32>>,
    }

    #[test_case("example1.ron")]
    #[test_case("case1.ron")]
    fn test_complicate_solution(test_case: &str) {
        let TestCase { mut input, answer } = from_file(
            PathBuf::new()
                .join("src/leetcode/algorithm_48")
                .join(test_case),
        );
        complicate::Solution::rotate(&mut input);
        assert_eq!(answer, input);
    }

    #[test_case("example1.ron")]
    #[test_case("case1.ron")]
    fn test_complicate_reverse_assign_solution(test_case: &str) {
        let TestCase { mut input, answer } = from_file(
            PathBuf::new()
                .join("src/leetcode/algorithm_48")
                .join(test_case),
        );
        complicate_reverse_assign::Solution::rotate(&mut input);
        assert_eq!(answer, input);
    }

    #[test_case("example1.ron")]
    #[test_case("case1.ron")]
    fn test_complicate_swap_assign_solution(test_case: &str) {
        let TestCase { mut input, answer } = from_file(
            PathBuf::new()
                .join("src/leetcode/algorithm_48")
                .join(test_case),
        );
        complicate_swap::Solution::rotate(&mut input);
        assert_eq!(answer, input);
    }

    #[test_case("example1.ron")]
    #[test_case("case1.ron")]
    fn test_mirror_solution(test_case: &str) {
        let TestCase { mut input, answer } = from_file(
            PathBuf::new()
                .join("src/leetcode/algorithm_48")
                .join(test_case),
        );
        mirror::Solution::rotate(&mut input);
        assert_eq!(answer, input);
    }
}
