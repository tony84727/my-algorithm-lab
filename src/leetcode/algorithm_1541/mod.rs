pub struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut o = 0;
        let mut i = 0;
        let mut ans = 0;
        let chars: Vec<char> = s.chars().collect();
        while i < chars.len() {
            match chars[i] {
                '(' => {
                    o += 1;
                }
                ')' => {
                    if i + 1 < chars.len() && chars[i] == ')' {
                        if o == 0 {
                            ans += 1;
                        } else {
                            o -= 1;
                        }
                        i += 2;
                        continue;
                    } else {
                        if o == 0 {
                            ans += 2;
                        } else {
                            o -= 1;
                            ans += 1;
                        }
                    }
                }
                _ => continue,
            }
            i += 1;
        }
        ans += 2 * o;
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("(()))" => 1; "example 1")]
    #[test_case("())" => 0; "example 2")]
    #[test_case("))())(" => 3; "example 3")]
    fn test_solution(s: &str) -> i32 {
        Solution::min_insertions(s.to_string())
    }
}
