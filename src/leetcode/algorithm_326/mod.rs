pub struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let l = (n as f32).log(3.0);
        3_i32.pow(l.floor() as u32) == n
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(27 => true; "example 1")]
    #[test_case(0 => false; "example 2")]
    #[test_case(-1 => false; "example 3")]
    #[test_case(1594322 => false; "case 1")]
    fn test_solution(n: i32) -> bool {
        Solution::is_power_of_three(n)
    }
}
