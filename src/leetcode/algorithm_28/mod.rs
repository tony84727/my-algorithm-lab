pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|x| x as i32).unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("sadbutsad", "sad" => 0; "example 1")]
    #[test_case("leetcode", "leeto" => -1; "example 2")]
    #[test_case("mississippi", "issip" => 4; "case 1")]
    fn test_solution(haystack: &str, needle: &str) -> i32 {
        Solution::str_str(haystack.to_owned(), needle.to_owned())
    }
}
