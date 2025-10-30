pub struct Solution;

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        if target.is_empty() {
            return 0;
        }
        let mut prev = 0;
        target.into_iter().fold(0, |acc, value| {
            let delta = (value - prev).max(0);
            prev = value;
            acc + delta
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,2,1] => 3; "example 1")]
    #[test_case(vec![3,1,1,2] => 4; "example 2")]
    #[test_case(vec![3,1,5,4,2] => 7; "example 3")]
    #[test_case(vec![3,4,2,5,6] => 8; "example 4")]
    fn test_solution(target: Vec<i32>) -> i32 {
        Solution::min_number_operations(target)
    }
}
