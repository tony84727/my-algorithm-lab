pub struct Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mask = !(0x7FFFFFFF << if n > 1 { (n - 1).ilog2() + 1 } else { 1 });
        ((n >> 1) ^ !n) & mask == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5 => true; "example 1")]
    #[test_case(7 => false; "example 2")]
    #[test_case(11 => false; "example 3")]
    #[test_case(10 => true; "case 1")]
    #[test_case(1 => true; "case 2")]
    fn test_solution(n: i32) -> bool {
        Solution::has_alternating_bits(n)
    }
}
