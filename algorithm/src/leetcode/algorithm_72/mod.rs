pub mod recursive;
pub mod recursive_cache;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("horse", "ros" => 3; "example 1")]
    #[test_case("intetion", "execution" => 5; "example 2")]
    #[test_case("hookse", "ros" => 4; "case 1")]
    fn test_recursive_solution(word1: &str, word2: &str) -> i32 {
        recursive::Solution::min_distance(word1.to_owned(), word2.to_owned())
    }

    #[test_case("horse", "ros" => 3; "example 1")]
    #[test_case("intetion", "execution" => 5; "example 2")]
    #[test_case("hookse", "ros" => 4; "case 1")]
    fn test_recursive_cache_solution(word1: &str, word2: &str) -> i32 {
        recursive_cache::Solution::min_distance(word1.to_owned(), word2.to_owned())
    }
}
