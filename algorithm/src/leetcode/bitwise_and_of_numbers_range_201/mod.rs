pub struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let most_significant_bit = (right as f32).log2().floor() as i32;
        let mut accumulated = 0;
        'check: for n in 0..=most_significant_bit {
            for x in left..=right {
                let mask = 1 << n;
                let bit = x & mask;
                if bit <= 0 {
                    continue 'check;
                }
            }
            accumulated |= 1 << n;
        }
        accumulated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5,7 => 4; "example 1")]
    #[test_case(0,0 => 0; "example 2")]
    #[test_case(1,2147483647 => 0; "example 3")]
    fn test_solution(left: i32, right: i32) -> i32 {
        Solution::range_bitwise_and(left, right)
    }
}
