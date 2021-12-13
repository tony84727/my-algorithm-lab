pub struct Solution;
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut current = None;
        let mut max = 0;
        let mut count = 1;
        for c in s.chars() {
            match &mut current {
                Some(current) => {
                    if c == *current {
                        count += 1;
                    } else {
                        *current = c;
                        count = 1;
                    }
                }
                None => {
                    current = Some(c);
                }
            }
            if count > max {
                max = count
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("leetcode" => 2; "example 1")]
    #[test_case("abbcccddddeeeeedcba" => 5; "example 2")]
    #[test_case("j" => 1; "case 1")]
    fn test_solution(s: &str) -> i32 {
        Solution::max_power(s.to_string())
    }
}
