pub mod half_loop;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("A man, a plan, a canal: Panama" => true; "example 1")]
    #[test_case("0P" => false; "case 1")]
    fn test_loop_solution(s: &str) -> bool {
        half_loop::Solution::is_palindrome(s.to_string())
    }
}
