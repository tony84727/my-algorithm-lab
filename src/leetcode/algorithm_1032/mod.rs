pub mod dfa;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use test_case::test_case;

    use crate::leetcode::common::test_utils::from_file;

    struct TestCase {
        words: Vec<String>,
        charaters: String,
        answer: Vec<bool>,
    }

    #[test_case("example_1.ron")]
    fn test_solution(test_case_path: &str) {
        let TestCase {
            words,
            charaters,
            answer,
        } = from_file(
            PathBuf::new()
                .join("src/leetcode/algorithm_1032")
                .join(test_case_path),
        );
    }
}
