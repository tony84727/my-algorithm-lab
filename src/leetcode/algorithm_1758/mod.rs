pub struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut zero_first = 0;
        let mut one_first = 0;
        for s in s.bytes() {
            if s == b'0' {
                one_first += 1;
            }
            if s == b'1' {
                zero_first += 1;
            }
            (zero_first, one_first) = (one_first, zero_first);
        }
        zero_first.min(one_first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("0100" => 1; "example 1")]
    #[test_case("10" => 0; "example 2")]
    #[test_case("1111" => 2; "example 3")]
    fn test_solution(s: &str) -> i32 {
        Solution::min_operations(String::from(s))
    }
}
