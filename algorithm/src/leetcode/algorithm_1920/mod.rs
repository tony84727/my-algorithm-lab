pub struct Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().map(|i| nums[*i as usize]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![0,2,1,5,3,4], vec![0,1,2,4,5,3]; "example 1")]
    #[test_case(vec![5,0,1,2,3,4], vec![4,5,0,1,2,3]; "example 2")]
    fn test_solution(nums: Vec<i32>, expected: Vec<i32>) {
        assert_eq!(expected, Solution::build_array(nums));
    }
}
