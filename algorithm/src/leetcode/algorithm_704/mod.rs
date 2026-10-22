pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut right = nums.len();
        let mut left = 0;
        while left < right {
            let mid = (right + left) / 2;
            let pivot = nums[mid];
            if pivot == target {
                return mid as i32;
            }
            if target > pivot {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![-1,0,3,5,9,12], 9 => 4; "example 1")]
    fn test_solution(nums: Vec<i32>, target: i32) -> i32 {
        Solution::search(nums, target)
    }
}
