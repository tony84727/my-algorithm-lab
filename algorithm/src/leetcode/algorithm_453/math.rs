pub struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = *nums.iter().min().unwrap();
        nums.into_iter().fold(0, |acc, c| acc + (c - min))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1, 2, 3] => 3; "example 1")]
    #[test_case(vec![1, 1, 1] => 0; "example 2")]
    fn test_min_moves(nums: Vec<i32>) -> i32 {
        Solution::min_moves(nums)
    }
}
