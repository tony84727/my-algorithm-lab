pub struct Solution;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let d = ((n as f32).log2() + 1.0) as usize;
        (1_i32 << d) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5 => 7; "example 1")]
    #[test_case(10 => 15; "example 2")]
    #[test_case(1 => 1; "case 1")]
    #[test_case(4 => 7; "case 2")]
    fn test_solution(n: i32) -> i32 {
        Solution::smallest_number(n)
    }
}
