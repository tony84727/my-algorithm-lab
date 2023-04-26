pub struct Solution;

impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        while num > 10 {
            num = num / 10 + num % 10;
        }
        num / 10 + num % 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(38 => 2; "example 1")]
    #[test_case(389 => 2)]
    #[test_case(12 => 3)]
    #[test_case(87 => 6)]
    #[test_case(8788 => 4)]
    fn test_solution(num: i32) -> i32 {
        Solution::add_digits(num)
    }
}
