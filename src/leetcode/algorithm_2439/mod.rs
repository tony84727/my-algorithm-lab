pub struct Solution;

impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut all = 0.0;
        for (i, &n) in nums.iter().enumerate() {
            all += n as f64;
            max = max.max((all / (i + 1) as f64).ceil() as i32);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,7,1,6] => 5; "example 1")]
    #[test_case(vec![10,1] => 10; "example 2")]
    #[test_case(vec![1,1,10] => 4; "case 1")]
    #[test_case(vec![1,10,4] => 6; "case 2")]
    #[test_case(vec![10,8,100] => 40; "case 3")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::minimize_array_value(nums)
    }
}
