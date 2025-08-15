pub struct Solution;

impl Solution {
    pub fn is_power_of_four(mut n: i32) -> bool {
        if n < 1 {
            return false;
        }
        while n % 4 == 0 {
            n /= 4;
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(16 => true; "example 1")]
    #[test_case(5 => false; "example 2")]
    #[test_case(1 => true; "example 3")]
    #[test_case(2 => false; "case 1")]
    fn test_solution(n: i32) -> bool {
        Solution::is_power_of_four(n)
    }
}
