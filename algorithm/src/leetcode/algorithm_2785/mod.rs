pub struct Solution;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut vowel_index = Vec::new();
        let mut vowels = Vec::new();
        for (i, c) in s.char_indices() {
            if ['a', 'e', 'i', 'o', 'u'].contains(&c.to_lowercase().next().unwrap()) {
                vowel_index.push(i);
                vowels.push(c);
            }
        }
        if vowels.is_empty() {
            return s;
        }
        vowels.sort_unstable();
        let mut chars: Vec<char> = s.chars().collect();
        for (i, &location) in vowel_index.iter().enumerate() {
            chars[location] = vowels[i];
        }
        String::from_iter(chars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("lEetcOde" => "lEOtcede"; "example 1")]
    #[test_case("lYmpH" => "lYmpH"; "example 2")]
    fn test_solution(s: &str) -> String {
        Solution::sort_vowels(String::from(s))
    }
}
