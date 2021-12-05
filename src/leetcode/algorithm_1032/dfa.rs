use std::collections::{HashMap, HashSet};

struct DFA {
    transactions: HashMap<char, i32>,
    accepts: HashSet<i32>,
}

struct StreamChecker {
    dfa: DFA,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {}

    fn query(&self, letter: char) -> bool {}
}
