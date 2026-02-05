pub struct Solution;

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        nums.iter()
            .enumerate()
            .map(|(i, j)| nums[((i as i32 + n + (j % n)) % n) as usize])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,-2,1,1] => vec![1,1,1,3]; "example 1")]
    #[test_case(vec![-1,4,-1] => vec![-1,-1,4]; "example 2")]
    #[test_case(vec![-10, -10] => vec![-10, -10]; "example 3")]
    fn test_solution(nums: Vec<i32>) -> Vec<i32> {
        Solution::construct_transformed_array(nums)
    }
}
