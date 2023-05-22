use std::{cmp::Reverse, collections::HashMap};

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts = HashMap::new();
        for n in nums.into_iter() {
            *counts.entry(n).or_default() += 1;
        }
        let mut counts: Vec<(i32, i32)> = counts.into_iter().collect();
        counts.sort_unstable_by_key(|(_element, count)| Reverse(*count));
        counts
            .into_iter()
            .take(k as usize)
            .map(|(element, _)| element)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,1,1,2,2,3], 2,vec![1,2]; "example 1")]
    fn test_solution(nums: Vec<i32>, k: i32, expected: Vec<i32>) {
        let expected: HashSet<i32> = HashSet::from_iter(expected.into_iter());
        let result = HashSet::from_iter(Solution::top_k_frequent(nums, k).into_iter());
        assert_eq!(expected, result);
    }
}
