pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return String::new();
        }
        let mut longest = None;
        let bytes = s.as_bytes();
        let mut record_result = |left: usize, right: usize| match longest {
            None => longest = Some((left, right)),
            Some((start, end)) if end - start < right - left => longest = Some((left, right)),
            _ => (),
        };
        let mut find = |left_start, right_start| {
            let mut right = right_start as i64;
            let mut left = left_start as i64;
            while left >= 0 && right < (s.len() as i64) {
                let l = left as usize;
                let r = right as usize;
                if bytes[l] != bytes[r] {
                    break;
                }
                record_result(l, r);
                left -= 1;
                right += 1;
            }
        };
        for i in 0..s.len() {
            find(i, i);
            find(i, i + 1);
        }
        let (start, end) = longest.unwrap();
        s[start..=end].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("babad" => "bab".to_string(); "example 1")]
    #[test_case("cbbd" => "bb".to_string(); "example 2")]
    #[test_case("ac" => "a".to_string(); "case 1")]
    #[test_case("abb" => "bb".to_string(); "case 2")]
    #[test_case("aacabdkacaa" => "aca".to_string(); "case 3")]
    fn test_solution(input: &str) -> String {
        Solution::longest_palindrome(input.to_string())
    }
}
