use std::{collections::{HashMap, HashSet}, vec};
use core::hash::Hash;

pub struct Solution;

fn get_alphabats() -> impl Iterator<Item = u8> {
    97..=122
}

#[derive(Hash, PartialEq, Eq)]
enum Label {
    Alphabet(char),
    Epsilon,
}

struct DFA {
    transactions: HashMap<char, u32>,
    acceptable_state_list: HashSet<u32>,
}

struct NFA {
    transaction_table_map: HashMap<u32, HashMap<Label, Vec<u32>>>,
    accept_state: u32,
}

impl NFA {
    fn build(pattern: &str) -> Self {
        let mut transaction_table_map = HashMap::new();
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

    fn get_closure_state(&self, state: u32, alphabet: char) -> CombinedState {
        CombinedState::new(self.closure(state, alphabet))
    }

    fn is_accetabpe(&self, state: u32) -> bool {
        self.accept_state == state
    }

    fn to_dfa(&self) -> DFA {
        let mut visited_state = HashSet::new();
        let first_state = CombinedState::new(vec![0]);
        visited_state.insert(first_state.clone());
        let mut stack = vec![first_state];
        while !stack.is_empty() {
            let state = stack.pop().unwrap();
        }
        DFA{
            acceptable_state_list: vec![]
            transactions:
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct CombinedState(HashSet<u32>);

impl Hash for CombinedState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for s in self.0.iter() {
            state.write_u32(*s);
        }
    }
}

impl CombinedState {
    fn new(states: Vec<u32>) -> Self {
        let mut states = HashSet::new();
        for s in states.into_iter() {
            states.insert(s);
        }
        Self(states)
    }
    fn is_acceptable_by_nfa(&self, nfa: &NFA) -> bool {
        self.0.contains(&nfa.accept_state)
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

    #[test]
    fn test_nfa_closure() {
        let nfa = NFA::build(".*.");
        assert_eq!(vec![1],nfa.closure(0, 'a'));
        assert_eq!(vec![1],nfa.closure(0, 'b'));
        assert_eq!(vec![1,2], nfa.closure(1, 'a'))
    }
}
