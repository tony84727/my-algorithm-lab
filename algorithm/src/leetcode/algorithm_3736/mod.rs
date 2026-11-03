pub struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let max = nums.iter().max().cloned().unwrap();
        nums.into_iter().map(|x| max - x).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2, 1, 3] => 3; "example 1")]
    #[test_case(vec![4, 4, 5] => 2; "example 2")]
    fn test_min_moves(nums: Vec<i32>) -> i32 {
        Solution::min_moves(nums)
    }
}
