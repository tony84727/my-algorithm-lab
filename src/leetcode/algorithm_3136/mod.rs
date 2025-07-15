pub struct Solution;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }
        let mut vowel = false;
        let mut consonant = false;
        for c in word.chars() {
            if !c.is_alphanumeric() {
                return false;
            }
            match c.to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    vowel = true;
                }
                _ if c.is_alphabetic() => {
                    consonant = true;
                }
                _ => (),
            }
        }
        vowel && consonant
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("234Adas" => true; "example 1")]
    #[test_case("b3" => false; "example 2")]
    #[test_case("ae$e" => false; "example 3")]
    #[test_case("AhI" => true; "case 1")]
    fn test_solution(input: &str) -> bool {
        Solution::is_valid(String::from(input))
    }
}
