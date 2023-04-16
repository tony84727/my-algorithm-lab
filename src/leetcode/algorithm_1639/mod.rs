pub mod divide_and_conquer;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&["acca", "bbbb", "caca"], "aba" => 6; "example 1")]
    fn test_divide_and_conquer(words: &[&'static str], target: &str) -> i32 {
        divide_and_conquer::Solution::num_ways(
            words.iter().map(|s| s.to_string()).collect(),
            target.to_owned(),
        )
    }
}
