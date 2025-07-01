pub struct Solution;

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut last: Option<char> = None;
        let mut posibility = 1;
        for w in word.chars() {
            if Some(w) == last {
                posibility += 1;
            }
            last = Some(w);
        }
        posibility
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abbccc" => 4; "example 1")]
    #[test_case("abcd" => 1; "example 2")]
    #[test_case("aaaa" => 4; "example 3")]
    fn test_solution(word: &'static str) -> i32 {
        Solution::possible_string_count(String::from(word))
    }
}
