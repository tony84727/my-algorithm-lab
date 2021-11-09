pub mod bitmask;
pub mod brute_force;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&["aaaa","asas","able","ability","actt","actor","access"], &["aboveyz","abrodyz","abslute","absoryz","actresz","gaswxyz"] => vec![1,1,3,2,4,0]; "example 1")]
    #[test_case(&["apple","pleas","please"], &["aelwxyz","aelpxyz","aelpsxy","saelpxy","xaelpsy"] => vec![0,1,3,2,0]; "case 1")]
    fn test_brute_force_solution(words: &[&str], puzzles: &[&str]) -> Vec<i32> {
        let words = words.iter().map(|s| s.to_string()).collect();
        let puzzles = puzzles.iter().map(|s| s.to_string()).collect();
        brute_force::Solution::find_num_of_valid_words(words, puzzles)
    }

    #[test_case(&["aaaa","asas","able","ability","actt","actor","access"], &["aboveyz","abrodyz","abslute","absoryz","actresz","gaswxyz"] => vec![1,1,3,2,4,0]; "example 1")]
    #[test_case(&["apple","pleas","please"], &["aelwxyz","aelpxyz","aelpsxy","saelpxy","xaelpsy"] => vec![0,1,3,2,0]; "case 1")]
    fn test_bitmap_solution(words: &[&str], puzzles: &[&str]) -> Vec<i32> {
        let words = words.iter().map(|s| s.to_string()).collect();
        let puzzles = puzzles.iter().map(|s| s.to_string()).collect();
        bitmask::Solution::find_num_of_valid_words(words, puzzles)
    }
}
