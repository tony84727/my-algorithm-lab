pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        Self::solve(&s1, &s2, &s3) || Self::solve(&s2, &s1, &s3)
    }

    fn solve(s1: &str, s2: &str, s3: &str) -> bool {
        if s1.is_empty() && s2.is_empty() && s3.is_empty() {
            return true;
        }
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        for (i, (j, k)) in s1.chars().zip(s3.chars()).enumerate() {
            if j != k {
                break;
            }
            if Self::solve(s2, &s1[i + 1..], &s3[i + 1..]) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("aabcc", "dbbca", "aadbbcbcac" => true; "example 1")]
    #[test_case("aabcc", "dbbca", "aadbbbaccc" => false; "example 2")]
    #[test_case("", "", "" => true; "example 3")]
    fn test_solution(s1: &'static str, s2: &'static str, s3: &'static str) -> bool {
        Solution::is_interleave(String::from(s1), String::from(s2), String::from(s3))
    }
}
