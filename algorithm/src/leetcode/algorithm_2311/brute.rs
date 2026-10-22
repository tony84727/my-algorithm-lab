pub struct Solution;

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let mut longest = 0;
        for i in 0..2_usize.pow(s.len() as u32) {
            if Self::binary_number_by_bitmap(&s, i) <= k {
                let len = i.count_ones() as i32;
                if len > longest {
                    longest = len
                }
            }
        }
        longest
    }

    fn binary_number_by_bitmap(s: &str, b: usize) -> i32 {
        let mut base = 0;
        let mut sum = 0;
        for (i, c) in s.chars().rev().enumerate() {
            let present = b & 1 << i > 0;
            if present {
                sum += c.to_digit(10).unwrap() as i32 * 2_i32.pow(base);
                base += 1;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1001010", 5 => 5; "example 1")]
    #[test_case("00101001", 1 => 6; "example 2")]
    #[test_case("0", 583196182 => 1; "example 3")]
    #[test_case("00101100", 8 => 6; "case 1")]
    fn test_solution(s: &str, k: i32) -> i32 {
        Solution::longest_subsequence(String::from(s), k)
    }
}
