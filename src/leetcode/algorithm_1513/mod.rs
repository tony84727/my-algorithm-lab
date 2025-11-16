use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let modulo = 1000000007;
        let n = s.len();
        let s = s.as_bytes();
        let mut count = 0;
        let start = RefCell::new(None);
        let mut add_count = |i: usize| {
            let mut start = start.borrow_mut();
            if let Some(start) = *start {
                let length = i - start;
                count = (count + (length + 1) * length / 2) % modulo;
            }
            *start = None;
        };
        for (i, &n) in s.iter().enumerate() {
            if n == b'0' {
                add_count(i);
            }
            let mut start = start.borrow_mut();
            if n == b'1' && start.is_none() {
                *start = Some(i);
            }
        }
        add_count(n);
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("0110111" => 9; "example 1")]
    #[test_case("101" => 2; "example 2")]
    #[test_case("111111" => 21; "example 3")]
    fn test_solution(s: &str) -> i32 {
        Solution::num_sub(String::from(s))
    }
}
