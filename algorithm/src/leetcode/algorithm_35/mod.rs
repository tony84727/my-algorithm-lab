pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len();
        let mut position = (start + end) / 2;

        while end > start {
            if nums[position] < target {
                start = position + 1;
            } else {
                end = position;
            }
            position = (start + end) / 2;
        }
        position as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,3,5,6], 5 => 2; "example 1")]
    #[test_case(vec![1,3,5,6], 2 => 1; "example 2")]
    #[test_case(vec![1,3,5,6], 7 => 4; "example 3")]
    #[test_case(vec![1,3,5,6], 0 => 0; "example 4")]
    #[test_case(vec![1], 0 => 0; "example 5")]
    fn test_solution(nums: Vec<i32>, target: i32) -> i32 {
        Solution::search_insert(nums, target)
    }
}
