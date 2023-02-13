pub struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (high - low) / 2 + if (low & 1) | (high & 1) != 0 { 1 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3,7 => 3; "example 1")]
    #[test_case(8,10 => 1; "example 2")]
    fn test_solution(low: i32, high: i32) -> i32 {
        Solution::count_odds(low, high)
    }
}
