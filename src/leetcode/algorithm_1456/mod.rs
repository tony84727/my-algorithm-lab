pub struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut counts = 0;
        let mut max = 0;
        let chars = s.chars().collect::<Vec<char>>();
        for &c in chars.iter().take(k as usize) {
            if Self::is_vowel(c) {
                counts += 1;
                max = max.max(counts);
            }
        }
        for (i, &c) in chars.iter().enumerate().skip(k as usize) {
            if Self::is_vowel(chars[i - (k as usize)]) {
                counts -= 1;
            }

            if Self::is_vowel(c) {
                counts += 1;
                max = max.max(counts);
            }
        }
        max
    }

    fn is_vowel(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abciiidef", 3 => 3; "example 1")]
    #[test_case("aeiou", 2 => 2; "example 2")]
    fn test_solution(s: &str, k: i32) -> i32 {
        Solution::max_vowels(s.to_owned(), k)
    }
}
