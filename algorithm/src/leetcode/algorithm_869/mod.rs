use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let target = Self::digit_frequency(n);
        for p in 0..=30_u32 {
            let frequency = Self::digit_frequency(2_i32.pow(p));
            if frequency == target {
                return true;
            }
        }
        false
    }
    fn digit_frequency(mut n: i32) -> HashMap<i32, usize> {
        let mut frequency = HashMap::new();
        while n != 0 {
            *frequency.entry(n % 10).or_default() += 1;
            n /= 10;
        }
        frequency
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1 => true; "example 1")]
    #[test_case(10 => false; "example 2")]
    fn test_solution(n: i32) -> bool {
        Solution::reordered_power_of2(n)
    }
}
