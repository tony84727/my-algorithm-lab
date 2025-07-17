pub struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }
        let mut max = 0;
        for r in 0..k {
            let length = Self::solve(&nums, k, r, None);
            max = max.max(length);
        }
        max
    }

    fn solve(nums: &[i32], k: i32, m: i32, last: Option<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        match last {
            Some(last) => {
                if (last + nums[0]) % k != m {
                    return Self::solve(&nums[1..], k, m, Some(last));
                }
                1 + Self::solve(&nums[1..], k, m, Some(nums[0]))
            }
            None => (1 + Self::solve(&nums[1..], k, m, Some(nums[0]))).max(Self::solve(
                &nums[1..],
                k,
                m,
                None,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,4,5], 2 => 5; "example 1")]
    #[test_case(vec![1,4,2,3,1,4], 3 => 4; "example 2")]
    #[test_case(vec![1,7,9], 10 => 2; "case 1")]
    #[test_case(vec![8,2,8], 5 => 3; "case 2")]
    #[test_case(vec![1,2,3,10,2], 6 => 3; "case 3")]
    fn test_solution(nums: Vec<i32>, k: i32) -> i32 {
        Solution::maximum_length(nums, k)
    }
}
