pub mod naive;
#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![
        "11110",
        "11010",
        "11000",
        "00000",
    ] => 1; "example 1")]
    #[test_case(vec![
        "111",
        "010",
        "111",
    ] => 1; "case 1")]
    #[test_case(vec!["10111", "10101", "11101"] => 1; "case 2")]
    #[test_case(vec!["11000", "11000","00100", "00011"] => 3; "case 3")]
    fn test_naive(map: Vec<&str>) -> i32 {
        let map = map
            .iter()
            .map(|row| row.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        naive::Solution::num_islands(map)
    }
}
