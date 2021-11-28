pub mod naive;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::path::PathBuf;
    use test_case::test_case;

    use crate::leetcode::common::test_utils;

    #[derive(Deserialize)]
    struct TestCase {
        input: Vec<Vec<i32>>,
        answer: Vec<Vec<i32>>,
    }

    #[test_case("example_1.ron")]
    fn test_solution(test_case_path: &str) {
        let file_path = PathBuf::new()
            .join("src/leetcode/algorithm_797")
            .join(test_case_path);
        let test_case: TestCase = test_utils::from_file(file_path);
        assert_eq!(
            test_case.answer,
            naive::Solution::all_paths_source_target(test_case.input)
        )
    }
}
