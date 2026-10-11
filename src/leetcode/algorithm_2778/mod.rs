pub struct Solution;

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        for (j, c) in nums.into_iter().enumerate() {
            if n.is_multiple_of(j + 1) {
                ans += c * c;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1, 2, 3, 4] => 21; "example 1")]
    #[test_case(vec![2, 7, 1, 19, 18, 3] => 63; "example 2")]
    fn test_sum_of_squares(nums: Vec<i32>) -> i32 {
        Solution::sum_of_squares(nums)
    }
}
