use std::collections::HashMap;

#[derive(Default)]
pub struct WordDictionary {
    trie: TrieNode,
}

impl WordDictionary {
    pub fn new() -> Self {
        Self {
            trie: TrieNode::new(false),
        }
    }

    pub fn add_word(&mut self, word: String) {
        let mut current = &mut self.trie;
        for c in word.chars() {
            current = current.children.entry(c).or_default();
        }
        current.terminal = true;
    }

    pub fn search(&self, word: String) -> bool {
        Self::trie_search(&self.trie, &word)
    }

    fn trie_search(node: &TrieNode, word: &str) -> bool {
        if word.is_empty() {
            return node.terminal;
        }
        let c = match word.chars().next() {
            Some(c) => c,
            None => return false,
        };
        match c {
            '.' => {
                for next in node.children.values() {
                    if Self::trie_search(next, &word[1..]) {
                        return true;
                    }
                }
                false
            }
            c => match node.children.get(&c) {
                Some(next_node) => Self::trie_search(next_node, &word[1..]),
                None => false,
            },
        }
    }
}

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    terminal: bool,
}

impl TrieNode {
    fn new(terminal: bool) -> Self {
        Self {
            children: HashMap::new(),
            terminal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WordDictionary;

    #[test]
    fn test_solution_basic() {
        let mut dictionary = WordDictionary::new();
        assert!(!dictionary.search("hello".to_owned()));
        dictionary.add_word("hello".to_owned());
        assert!(dictionary.search("hello".to_owned()));
        assert!(dictionary.search("hel.o".to_owned()));
    }

    #[test]
    fn test_solution_example1() {
        let mut dictionary = WordDictionary::new();
        dictionary.add_word("bad".to_owned());
        dictionary.add_word("dad".to_owned());
        dictionary.add_word("mad".to_owned());
        assert!(!dictionary.search("pad".to_owned()));
        assert!(dictionary.search("bad".to_owned()));
        assert!(dictionary.search(".ad".to_owned()));
        assert!(dictionary.search("b..".to_owned()));
    }
}
