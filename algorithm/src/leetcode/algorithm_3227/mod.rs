pub struct Solution;

impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        if s.is_empty() {
            return false;
        }
        s.chars().any(|a| ['a', 'e', 'i', 'o', 'u'].contains(&a))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("leetcoder" => true; "example 1")]
    #[test_case("bbcd" => false; "example 2")]
    fn test_solution(s: &str) -> bool {
        Solution::does_alice_win(String::from(s))
    }
}
