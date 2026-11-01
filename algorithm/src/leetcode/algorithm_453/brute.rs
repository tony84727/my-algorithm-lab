pub struct Solution;

impl Solution {
    pub fn min_moves(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        let mut count = 0;
        while nums[0] != nums[n - 1] {
            for n in nums.iter_mut().take(n - 1) {
                *n += 1;
            }
            count += 1;
            nums.sort_unstable();
        }
        count
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
