pub mod convergence;
#[cfg(test)]
mod tests {
    use crate::leetcode::common::test_utils::from_file;
    use std::path::PathBuf;

    use super::*;
    use serde::Deserialize;
    use test_case::test_case;

    #[derive(Deserialize)]
    struct TestCase {
        input: Vec<Vec<i32>>,
        answer: Vec<Vec<i32>>,
    }

    #[test_case("example1.ron")]
    #[test_case("case1.ron")]
    #[test_case("case2.ron")]
    #[test_case("case3.ron")]
    #[test_case("case4.ron")]
    #[test_case("case5.ron")]
    fn test_convergence_solution(path: &str) {
        let TestCase { input, answer }: TestCase =
            from_file(PathBuf::new().join("src/leetcode/algorithm_56").join(path));
        assert_eq!(answer, convergence::Solution::merge(input));
    }
}
