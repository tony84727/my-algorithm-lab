pub struct Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let a = n / 7;
        let b = n % 7;
        7 * (a * (7 + a)) / 2 + b * (2 * a + b + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(4 => 10; "example 1")]
    #[test_case(10 => 37; "example 2")]
    #[test_case(20 => 96; "example 3")]
    fn test_solution(n: i32) -> i32 {
        Solution::total_money(n)
    }
}
