pub struct Solution;

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut digits: Vec<i32> = s.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
        while digits.len() > 2 {
            let mut next = Vec::new();
            for (i, &a) in digits.iter().enumerate().take(digits.len() - 1) {
                next.push((a + digits[i + 1]) % 10);
            }
            digits = next;
        }
        digits[0] == digits[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("3902" => true; "example 1")]
    #[test_case("34789" => false; "example 2")]
    fn test_solution(s: &str) -> bool {
        Solution::has_same_digits(String::from(s))
    }
}
