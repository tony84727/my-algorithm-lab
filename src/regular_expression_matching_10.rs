use std::{
    ascii::EscapeDefault,
    collections::{HashMap, HashSet},
    os::macos::raw::stat,
    vec,
};

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
            transaction_table_map.insert(0, transaction);
        } else {
            for c in pattern.chars() {
                match c {
                    '*' => {
                        transaction_table_map
                            .entry(last_state)
                            .or_insert(HashMap::new())
                            .entry(Label::Epsilon)
                            .or_insert(vec![])
                            .push(last_state);
                    }
                    '.' => {
                        let self_state = last_state;
                        last_state += 1;
                        transaction_table_map
                            .entry(self_state)
                            .or_insert(HashMap::new())
                            .entry(Label::Epsilon)
                            .or_insert(vec![])
                            .push(last_state);
                    }
                    _ => {
                        let self_state = last_state;
                        last_state += 1;
                        transaction_table_map
                            .entry(self_state)
                            .or_insert(HashMap::new())
                            .entry(Label::Alphabet(c))
                            .or_insert(vec![])
                            .push(last_state);
                    }
                };
            }
        }

        return Self {
            transaction_table_map: transaction_table_map,
            accept_state: last_state,
        };
    }

    fn closure(&self, current_state: u32, alphabet: char) -> Vec<u32> {
        let mut state_list = vec![];
        let transaction = self.transaction_table_map.get(&current_state).unwrap();
        if let Some(epsilon) = transaction.get(&Label::Epsilon) {
            for next in epsilon.iter() {
                state_list.push(*next);
            }
        }
        if let Some(next_state_list) = transaction.get(&Label::Alphabet(alphabet)) {
            for next in next_state_list.iter() {
                state_list.push(*next);
            }
        }
        state_list
    }
}

struct DFA {
    transactions: HashMap<char, u32>,
    acceptable_state_list: HashSet<u32>,
}

impl From<NFA> for DFA {
    fn from(nfa: NFA) -> Self {
        let mut stack =
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
