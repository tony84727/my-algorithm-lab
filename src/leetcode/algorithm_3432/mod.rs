pub struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left = 0;
        let mut right = nums.iter().sum::<i32>();
        let mut count = 0;
        for n in nums.into_iter().take(n - 1) {
            left += n;
            right -= n;
            if (right - left) % 2 == 0 {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![10,10,3,7,6] => 4; "example 1")]
    #[test_case(vec![1,2,2] => 0; "example 2")]
    #[test_case(vec![2,4,6,8] => 3; "example 3")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::count_partitions(nums)
    }
}
