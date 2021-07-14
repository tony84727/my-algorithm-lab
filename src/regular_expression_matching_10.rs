use std::{ascii::EscapeDefault, collections::HashMap, os::macos::raw::stat, vec};

pub struct Solution;

fn get_alphabats() -> impl Iterator<Item = u8> {
    97..=122
}

#[derive(Hash, PartialEq, Eq)]
enum Label {
    Alphabet(char),
    Epsilon,
}

struct NFA {
    transaction_table_map: HashMap<u32, HashMap<Label, Vec<u32>>>,
    accept_state: u32,
}

impl NFA {
    fn build(pattern: &str) -> Self {
        let transaction_table_map = HashMap::new();
        let mut last_state = 0;
        if pattern.len() <= 0 {
            let mut transaction = HashMap::new();
            transaction_table_map.insert(0, v)
            transaction_table_list
                .entry(Label::Epsilon)
                .or_insert(vec![])
                .push(0);
            return Self {
                transaction_table_map: transaction_table_list,
                accept_state: 0,
            };
        }
        for c in pattern.chars() {
            match c {
                '*' => {
                    transaction_table_list
                        .entry(Label::Epsilon)
                        .or_insert(vec![])
                        .push(last_state);
                }
                '.' => {
                    last_state += 1;
                    transaction_table_list
                        .entry(Label::Epsilon)
                        .or_insert(vec![])
                        .push(last_state);
                }
                _ => {
                    last_state += 1;
                    transaction_table_list
                        .entry(Label::Alphabet(c))
                        .or_insert(vec![])
                        .push(last_state);
                }
            };
        }
        return Self {
            transaction_table_map: transaction_table_list,
            accept_state: last_state,
        };
    }

    fn closure(&self, current_state: u32, alphabet: char) -> Vec<u32> {
        let mut state_list = vec![];
        if let Some(epsilon) = self.transaction_table_map.get(&Label::Epsilon) {
            for next in epsilon.iter() {
                state_list.push(*next);
            }
        }
        state_list
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
        assert_eq!(
            expected,
            Solution::is_match(input.to_string(), pattern.to_string())
        )
    }
}
