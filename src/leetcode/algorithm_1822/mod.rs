pub struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(1, |a, c| {
            a * match c {
                0 => 0,
                x if x > 0 => 1,
                _ => -1,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![-1,-2,-3,-4,3,2,1] => 1; "example 1")]
    #[test_case(vec![1,5,0,2,-3] => 0; "example 2")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::array_sign(nums)
    }
}
