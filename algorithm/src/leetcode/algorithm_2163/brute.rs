pub struct Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        let n = nums.len() / 3;
        Self::minimum_difference_after_remove(&nums, n)
    }

    fn minimum_difference_after_remove(nums: &[i64], remaining: usize) -> i64 {
        if remaining == 0 {
            let mid = nums.len() / 2;
            let left_sum: i64 = nums[..mid].iter().sum();
            let right_sum: i64 = nums[mid..].iter().sum();
            return left_sum - right_sum;
        }
        let mut min = Self::minimum_difference_after_remove(&nums[1..], remaining - 1);
        for i in 1..nums.len() {
            min = min.min(Self::minimum_difference_after_remove(
                &[&nums[..i], &nums[i + 1..]].concat(),
                remaining - 1,
            ))
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,1,2] => -1; "example 1")]
    #[test_case(vec![7,9,5,8,1,3] => 1; "example 2")]
    fn test_solution(nums: Vec<i32>) -> i64 {
        Solution::minimum_difference(nums)
    }
}
