use std::collections::{HashMap, HashSet};

pub struct Solution;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut case_sensitive_index = HashSet::new();
        let mut case_insensitive_index = HashMap::new();
        let max_prioirty = wordlist.len();
        for (i, w) in wordlist.into_iter().enumerate() {
            case_sensitive_index.insert(w.clone());
            let insensitive = w.to_lowercase();
            if case_insensitive_index.contains_key(&insensitive) {
                continue;
            }
            case_insensitive_index.insert(insensitive, (i, w));
        }

        let mut corrected = Vec::with_capacity(queries.len());
        for q in queries.into_iter() {
            if case_sensitive_index.contains(&q) {
                corrected.push(q);
                continue;
            }
            if let Some((_p, correct)) = case_insensitive_index.get(&q.to_lowercase()) {
                corrected.push(correct.clone());
                continue;
            }
            let mut answer = String::new();
            let mut priority = max_prioirty;
            let variants = Self::vowel_variants(&q.to_lowercase());
            for variant in variants.into_iter() {
                if let Some((p, correct)) = case_insensitive_index.get(&variant) {
                    if *p < priority {
                        answer = correct.clone();
                        priority = *p;
                    }
                }
            }
            corrected.push(answer)
        }
        corrected
    }

    fn vowel_variants(s: &str) -> Vec<String> {
        let mut variants = Vec::new();
        for (i, c) in s.char_indices() {
            if !VOWELS.contains(&c) {
                continue;
            }
            let rests = Self::vowel_variants(&s[i + 1..]);
            for v in VOWELS.iter() {
                let mut current = String::from(&s[..i]);
                current.push(*v);
                for rest in rests.iter() {
                    variants.push(current.clone() + rest);
                }
            }
        }
        if variants.is_empty() {
            vec![String::from(s)]
        } else {
            variants
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec!["KiTe","kite","hare","Hare"], vec!["kite","Kite","KiTe","Hare","HARE","Hear","hear","keti","keet","keto"] => vec!["kite","KiTe","KiTe","Hare","hare","","","KiTe","","KiTe"]; "example 1")]
    #[test_case(vec!["yellow"], vec!["YellOw"] => vec!["yellow"]; "example 2")]
    #[test_case(vec!["zeo","Zuo"], vec!["zuo"] => vec!["Zuo"]; "case 1")]
    fn test_solution(wordlist: Vec<&str>, queries: Vec<&str>) -> Vec<String> {
        Solution::spellchecker(
            wordlist.into_iter().map(String::from).collect(),
            queries.into_iter().map(String::from).collect(),
        )
    }

    #[test_case(String::from("HARE"))]
    fn test_vowel_variants(s: String) {
        let variants = Solution::vowel_variants(&s.to_lowercase());
        assert!(variants.contains(&s.to_lowercase()));
    }
}
