pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn merging(intervals: &mut Vec<Vec<i32>>) {
            intervals.sort_unstable_by_key(|item| item[0]);
            let mut current = 0;
            while current < intervals.len() - 1 {
                if intervals[current][1] >= intervals[current + 1][0] {
                    intervals[current + 1][0] = intervals[current][0];
                    intervals[current + 1][1] =
                        intervals[current][1].max(intervals[current + 1][1]);
                    intervals.remove(current);
                }
                current += 1;
            }
        }
        loop {
            let before = intervals.len();
            merging(&mut intervals);
            if before == intervals.len() {
                break;
            }
        }
        intervals
    }
}

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
    fn test_merge_solution(path: &str) {
        let TestCase { input, answer }: TestCase =
            from_file(PathBuf::new().join("src/leetcode/algorithm_56").join(path));
        assert_eq!(answer, Solution::merge(input));
    }
}
