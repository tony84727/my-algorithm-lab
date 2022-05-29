pub mod half_loop;
pub mod reverse;

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;
    use test_case::test_case;

    #[test_case("A man, a plan, a canal: Panama" => true; "example 1")]
    #[test_case("0P" => false; "case 1")]
    fn test_loop_solution(s: &str) -> bool {
        half_loop::Solution::is_palindrome(s.to_string())
    }

    quickcheck! {
        fn quickcheck_loop_soltuion(s: String) -> bool {
            let chars: Vec<char> = s.to_lowercase().chars().filter(|c| ('a'..='z').contains(c) || ('0'..='9').contains(c)).collect();
            let reversed = {
                let mut chars = chars.clone();
                chars.reverse();
                chars
            };
            (chars == reversed) == half_loop::Solution::is_palindrome(s)
        }
    }
    quickcheck! {
        fn quickcheck_xor_soltuion(s: String) -> bool {
            let chars: Vec<char> = s.to_lowercase().chars().filter(|c| ('a'..='z').contains(c) || ('0'..='9').contains(c)).collect();
            let reversed = {
                let mut chars = chars.clone();
                chars.reverse();
                chars
            };
            (chars == reversed) == reverse::Solution::is_palindrome(s)
        }
    }
}
