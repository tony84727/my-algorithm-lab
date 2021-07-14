use std::{collections::HashMap, vec};

pub struct Solution;

fn get_alphabats() -> impl Iterator<Item=u8> {
    97..=122
}

struct NFA {
    transaction_table: HashMap<u8, Vec<u32>>,
    accept_states: Vec<u32>,
}

impl NFA {
    fn build(pattern: &str) -> Self {
        let mut transaction_table = HashMap::new();
        let mut accept_states = vec![];
        if pattern.len() <= 0 {
            accept_states.push(0);
            for c in get_alphabats() {
                transaction_table.entry(c).or_insert(vec![]).push(0);
            }
            return Self {
                transaction_table,
                accept_states,
            }
        }
        for c in pattern.chars() {
            match c {
                '*' => {
                    for c in get_alphabats() {

                    }
                }
            }
        }
        return Self {
            transaction_table
        }
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("aa", "a", true; "example_1")]
    #[test_case("aa", "a*", true; "example_2")]
    #[test_case("ab", ".*", true; "example_3")]
    #[test_case("mississippi", "mis*is*p*.", false; "example_4")]
    fn test_is_match(input: &str, pattern: &str, expected: bool) {
        assert_eq!(expected, Solution::is_match(input.to_string(), pattern.to_string()))
    }
}