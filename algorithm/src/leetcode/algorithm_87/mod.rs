pub mod recursive;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("great", "rgeat" => true; "example 1")]
    #[test_case("abcdef", "caebd" => false; "example 2")]
    #[test_case("abcdbdacbdac", "bdacabcdbdac" => true; "case 1")]
    fn test_recursive_solution(s1: &'static str, s2: &'static str) -> bool {
        recursive::Solution::is_scramble(s1.to_owned(), s2.to_owned())
    }
}
