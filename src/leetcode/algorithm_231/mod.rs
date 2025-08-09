pub struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        n.count_ones() == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1 => true; "example 1")]
    #[test_case(16 => true; "example 2")]
    #[test_case(3 => false; "example 3")]
    #[test_case(-2147483648 => false; "case 1")]
    fn test_solution(n: i32) -> bool {
        Solution::is_power_of_two(n)
    }
}
