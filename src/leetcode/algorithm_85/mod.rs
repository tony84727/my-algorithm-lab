pub mod brute_force;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leetcode::common::test_utils::from_file;
    use serde::Deserialize;
    use std::path::PathBuf;
    use test_case::test_case;

    #[derive(Deserialize)]
    struct TestCase {
        input: Vec<Vec<char>>,
        answer: i32,
    }

    #[test_case("example_1.ron")]
    #[test_case("case_1.ron")]
    #[test_case("case_2.ron")]
    #[test_case("case_3.ron")]
    #[test_case("case_4.ron")]
    fn test_brute_force_solution(test_case_path: &str) {
        let TestCase { input, answer } = from_file(
            PathBuf::new()
                .join("src/leetcode/algorithm_85")
                .join(test_case_path),
        );
        assert_eq!(answer, brute_force::Solution::maximal_rectangle(input));
    }
}
