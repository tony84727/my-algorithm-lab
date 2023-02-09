use std::{
    collections::{HashMap, HashSet},
    ops::Sub,
};

pub struct Solution;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut dictionary = HashMap::<char, HashSet<String>>::new();
        for i in ideas.iter() {
            dictionary
                .entry(i.chars().next().unwrap())
                .or_default()
                .insert(i[1..].to_string());
        }
        let mut combination = 0;
        for (&a, a_suffixes) in dictionary.iter() {
            for (_b, b_suffixes) in dictionary.iter().filter(|(&k, _v)| k != a) {
                let valid_a_suffixes = a_suffixes.sub(b_suffixes).len() as i64;
                let valid_b_suffixes = b_suffixes.sub(a_suffixes).len() as i64;
                combination += valid_a_suffixes * valid_b_suffixes;
            }
        }
        combination
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&["coffee", "donuts", "time", "toffee"] => 6; "example 1")]
    fn test_solution(ideas: &[&str]) -> i64 {
        Solution::distinct_names(ideas.iter().map(|x| x.to_string()).collect())
    }
}
