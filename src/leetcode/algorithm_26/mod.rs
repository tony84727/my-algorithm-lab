pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,1,2] => 2; "example 1")]
    #[test_case(vec![0,0,1,1,1,2,2,3,3,4] => 5; "example 2")]
    fn test_solution(mut nums: Vec<i32>) -> i32 {
        Solution::remove_duplicates(&mut nums)
    }
}
