use std::collections::{HashMap, HashSet};

pub struct Solution;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut case_sensitive_index = HashSet::new();
        let mut case_insensitive_index = HashMap::new();
        let mut devowel_index = HashMap::new();
        for w in wordlist.into_iter() {
            case_sensitive_index.insert(w.clone());
            let insensitive: String = w.to_lowercase();
            let devowel = Self::devowel(&insensitive);
            case_insensitive_index
                .entry(insensitive)
                .or_insert_with(|| w.clone());
            devowel_index.entry(devowel).or_insert(w);
        }

        let mut corrected = Vec::with_capacity(queries.len());
        for q in queries.into_iter() {
            if case_sensitive_index.contains(&q) {
                corrected.push(q);
                continue;
            }
            if let Some(c) = case_insensitive_index.get(&q.to_lowercase()) {
                corrected.push(c.clone());
                continue;
            }
            corrected.push(match devowel_index.get(&Self::devowel(&q.to_lowercase())) {
                Some(x) => x.clone(),
                None => String::new(),
            });
        }
        corrected
    }

    fn devowel(s: &str) -> String {
        s.chars()
            .map(|x| if VOWELS.contains(&x) { '*' } else { x })
            .collect()
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
}
