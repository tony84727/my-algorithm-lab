pub struct Solution;

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let mut power = 0;
        while 1 << (power + 1) <= n {
            power += 1;
        }
        (1 << (power + 1)) - 1 - Self::minimum_one_bit_operations((1 << power) ^ n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3 => 2; "example 1")]
    #[test_case(6 => 4; "example 2")]
    fn test_solution(n: i32) -> i32 {
        Solution::minimum_one_bit_operations(n)
    }
}
