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
    use test::Bencher;
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

    fn create_large_matrix(size: usize) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; size]; size];
        for row in 0..matrix.len() {
            for column in 0..matrix.len() {
                matrix[row][column] = (column % 2) as i32;
            }
        }
        matrix
    }

    #[bench]
    fn mirror_solution(b: &mut Bencher) {
        let mut matrix = create_large_matrix(100);
        b.iter(move || mirror::Solution::rotate(&mut matrix));
    }

    #[bench]
    fn complicate_reverse_assign(b: &mut Bencher) {
        let mut matrix = create_large_matrix(100);
        b.iter(move || complicate_reverse_assign::Solution::rotate(&mut matrix));
    }

    #[bench]
    fn complicate(b: &mut Bencher) {
        let mut matrix = create_large_matrix(100);
        b.iter(move || complicate::Solution::rotate(&mut matrix));
    }

    #[bench]
    fn complicate_swap(b: &mut Bencher) {
        let mut matrix = create_large_matrix(100);
        b.iter(move || complicate_swap::Solution::rotate(&mut matrix));
    }
}
