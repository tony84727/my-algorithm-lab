use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn remove_anagrams(mut words: Vec<String>) -> Vec<String> {
        let mut freqencies = Vec::with_capacity(words.len());
        for w in words.iter() {
            let mut frequency: BTreeMap<char, usize> = BTreeMap::new();
            for c in w.chars() {
                *frequency.entry(c).or_default() += 1;
            }
            freqencies.push(frequency);
        }
        loop {
            let mut to_remove = None;
            for (i, f) in freqencies.iter().enumerate().take(freqencies.len() - 1) {
                if *f == freqencies[i + 1] {
                    to_remove = Some(i + 1)
                }
            }
            match to_remove {
                None => break,
                Some(index) => {
                    words.remove(index);
                    freqencies.remove(index);
                }
            };
        }
        words
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec!["abba","baba","bbaa","cd","cd"] => vec!["abba","cd"]; "example 1")]
    #[test_case(vec!["a","b","c","d","e"] => vec!["a","b","c","d","e"]; "example 2")]
    #[test_case(vec!["a","b","a"] => vec!["a","b","a"]; "case 1")]
    fn test_solution(words: Vec<&str>) -> Vec<String> {
        Solution::remove_anagrams(words.into_iter().map(String::from).collect())
    }
}
