pub struct Solution;

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut curr_ones: i64 = 0;
        let mut ops: i64 = 0;
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i] == b'1' {
                curr_ones += 1;
                i += 1;
            } else {
                if curr_ones > 0 {
                    ops += curr_ones;
                }
                while i < bytes.len() && bytes[i] == b'0' {
                    i += 1;
                }
            }
        }

        ops as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1001101" => 4; "example 1")]
    #[test_case("000111" => 0; "example 2")]
    #[test_case("010010" => 3; "case 1")]
    #[test_case("11000" => 2; "case 2")]
    #[test_case("101010" => 6; "case 3")]
    fn test_solution(s: &str) -> i32 {
        Solution::max_operations(String::from(s))
    }
}
