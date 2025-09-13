use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut vowels: HashMap<char, usize> = HashMap::new();
        let mut consonants: HashMap<char, usize> = HashMap::new();
        let mut max_vowel_count = 0;
        let mut max_consonant_count = 0;
        for c in s.chars() {
            if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
                let count = vowels.entry(c).or_default();
                *count += 1;
                max_vowel_count = max_vowel_count.max(*count);
                continue;
            }
            let count = consonants.entry(c).or_default();
            *count += 1;
            max_consonant_count = max_consonant_count.max(*count);
        }

        (max_vowel_count + max_consonant_count) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("successes" => 6; "example 1")]
    #[test_case("aeiaeia" => 3; "example 2")]
    fn test_solution(s: &str) -> i32 {
        Solution::max_freq_sum(String::from(s))
    }
}
