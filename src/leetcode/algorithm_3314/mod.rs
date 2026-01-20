pub struct Solution;

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .map(|x| (0..x).find(|&i| i | (i + 1) == x).unwrap_or(-1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,3,5,7] => vec![-1,1,4,3]; "example 1")]
    #[test_case(vec![11,13,31] => vec![9,12,15]; "example 2")]
    fn test_solution(nums: Vec<i32>) -> Vec<i32> {
        Solution::min_bitwise_array(nums)
    }
}
