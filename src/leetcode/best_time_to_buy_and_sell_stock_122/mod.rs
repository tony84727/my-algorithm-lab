pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![7,1,5,3,6,4] => 7; "example 1")]
    fn test_solution(prices: Vec<i32>) -> i32 {
        Solution::max_profit(prices)
    }
}
