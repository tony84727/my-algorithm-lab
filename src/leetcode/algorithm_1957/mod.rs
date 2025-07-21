pub struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let mut count = 0;
        let mut last = ' ';
        let mut result = String::with_capacity(s.len());
        for c in s.chars() {
            if c == last {
                count += 1;
                if count >= 3 {
                    continue;
                }
            } else {
                count = 1;
                last = c;
            }
            result.push(c);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("leeetcode" => "leetcode"; "example 1")]
    #[test_case("aaabaaaa" => "aabaa"; "example 2")]
    fn test_soltuion(input: &'static str) -> String {
        Solution::make_fancy_string(String::from(input))
    }
}
