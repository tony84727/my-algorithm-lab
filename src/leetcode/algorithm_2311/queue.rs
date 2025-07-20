use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let mut ones: VecDeque<usize> = VecDeque::new();
        let mut current_length = 0;
        let mut max = 0;
        let mut sum = 0;
        for (i, c) in s.chars().enumerate() {
            current_length += 1;
            sum *= 2;
            let digit = c.to_digit(10).unwrap();
            sum += digit as i32;
            if digit == 1 {
                ones.push_back(i);
            }
            if sum <= k && current_length > max {
                max = current_length;
                continue;
            }
            sum -= 2_i32.pow((i - ones.pop_front().unwrap()) as u32);
            current_length -= 1;
        }
        max
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
