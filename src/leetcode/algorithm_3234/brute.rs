pub struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let n = s.len();
        let s: Vec<u8> = s.bytes().collect();
        let mut count = 0;
        for i in 0..n {
            let mut ones: i32 = 0;
            let mut zeros: i32 = 0;
            for &c in s.iter().skip(i) {
                if c == b'1' {
                    ones += 1;
                } else {
                    zeros += 1;
                }
                if ones >= zeros.pow(2) {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("00011" => 5; "example 1")]
    #[test_case("101101" => 16; "example 2")]
    fn test_solution(s: &str) -> i32 {
        Solution::number_of_substrings(String::from(s))
    }
}
