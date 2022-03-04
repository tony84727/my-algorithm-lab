pub mod recursive;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec!["ABCE", "SFCS", "ADEE"], "ABCCED" => true; "example 1")]
    #[test_case(vec!["ABCE", "SFCS", "ADEE"], "ABCB" => false; "case 1")]
    #[test_case(vec!["ABCE", "SFES", "ADEE"], "ABCEFSADEESE" => true; "case 2")]
    fn test_recursive_solution(board: Vec<&str>, word: &str) -> bool {
        let board = board.into_iter().map(|s| s.chars().collect()).collect();
        recursive::Solution::exist(board, word.to_string())
    }
}
