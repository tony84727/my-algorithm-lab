pub struct Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let x = n ^ (n >> 1);
        (x & x.wrapping_add(1)) == 0
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
