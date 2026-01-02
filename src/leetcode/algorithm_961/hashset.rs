use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut visited: HashSet<i32> = HashSet::new();
        for x in nums.into_iter() {
            if visited.contains(&x) {
                return x;
            }
            visited.insert(x);
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,3] => 3; "example 1")]
    #[test_case(vec![2,1,2,5,3,2] => 2; "example 2")]
    #[test_case(vec![5,1,5,2,5,3,5,4] => 5; "example 3")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::repeated_n_times(nums)
    }
}
