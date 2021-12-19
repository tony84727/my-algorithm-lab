use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn search(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
            if target <= 0 {
                return vec![];
            }
            let mut answers = vec![];
            for &candidate in candidates.iter() {
                if candidate == target {
                    answers.push(vec![candidate]);
                }
                let combinations = search(candidates, target - candidate);
                for mut combination in combinations.into_iter() {
                    if combination.is_empty() {
                        continue;
                    }
                    combination.push(candidate);
                    combination.sort_unstable();
                    answers.push(combination)
                }
            }
            answers
        }
        let answers = search(&candidates, target);
        let mut set = HashSet::new();
        for answer in answers.into_iter() {
            set.insert(answer);
        }
        set.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::common::test_utils::from_file;

    use super::*;
    use serde::Deserialize;
    use std::{iter::FromIterator, path::PathBuf};
    use test_case::test_case;

    type Input = (Vec<i32>, i32);

    #[derive(Deserialize)]
    struct TestCase {
        input: Input,
        answer: Vec<Vec<i32>>,
    }

    #[test_case("example1.ron")]
    #[test_case("case1.ron")]
    fn test_solution(test_case: &str) {
        let TestCase {
            input: (candidcates, target),
            answer,
        } = from_file(
            PathBuf::new()
                .join("src/leetcode/algorithm_39")
                .join(test_case),
        );
        let answer: HashSet<Vec<i32>> = HashSet::from_iter(answer.into_iter());
        let actual = HashSet::from_iter(Solution::combination_sum(candidcates, target).into_iter());
        assert_eq!(answer, actual)
    }
}
