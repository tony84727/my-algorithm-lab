pub struct Solution;

impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.into_iter().rev().take(2).map(|x| x - 1).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3, 4, 5, 2] => 12; "example 1")]
    #[test_case(vec![1, 5, 4, 5] => 16; "example 2")]
    #[test_case(vec![3, 7] => 12; "example 3")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::max_product(nums)
    }
}
