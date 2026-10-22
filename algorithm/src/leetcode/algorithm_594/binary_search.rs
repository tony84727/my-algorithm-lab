pub struct Solution;

impl Solution {
    pub fn find_lhs(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut answer = 0;
        for (i, min) in nums.iter().enumerate().take(nums.len() - 1) {
            let max = *min + 1;
            let mut right = nums.len() - 1;
            let mut left = i + 1;
            let mut mid = (right + left).div_ceil(2);
            while right > left {
                if nums[mid] <= max {
                    left = mid;
                }
                if nums[mid] > max {
                    right = mid - 1;
                }
                if right == left {
                    mid = right;
                    break;
                }
                mid = (right + left).div_ceil(2);
            }
            if nums[mid] == max {
                let length = mid - i + 1;
                if length > answer {
                    answer = length;
                }
            }
        }
        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,3,2,2,5,2,3,7] => 5; "example 1")]
    #[test_case(vec![1,2,3,4] => 2; "example 2")]
    #[test_case(vec![1,1,1,1] => 0; "example 3")]
    #[test_case(vec![-3,-1,-1,-1,-3,-2] => 4;"case 1")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::find_lhs(nums)
    }
}
