pub mod slow;

#[cfg(test)]
mod tests {
    use crate::leetcode::common::test_utils;

    use super::*;
    use serde::Deserialize;
    use std::path::PathBuf;
    use test_case::test_case;

    #[derive(Deserialize)]
    struct TestCase {
        input: Vec<Vec<String>>,
        answer: Vec<Vec<String>>,
    }

    #[test_case("example_1.ron")]
    #[test_case("case_1.ron")]
    fn test_solution(test_case_path: &str) {
        let test_case: TestCase = test_utils::from_file(
            PathBuf::new()
                .join("src/leetcode/algorithm_721")
                .join(test_case_path),
        );
        assert_eq!(
            test_case.answer,
            slow::Solution::accounts_merge(test_case.input)
        );
    }
}
