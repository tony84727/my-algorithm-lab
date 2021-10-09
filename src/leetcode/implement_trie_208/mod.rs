use std::collections::HashMap;

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(String::new()),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            let prefix = node.prefix.clone() + c.to_string().as_str();
            node = node
                .children
                .entry(c)
                .or_insert_with(|| TrieNode::new(prefix));
        }
        node.word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            match node.children.get(&c) {
                Some(next) => {
                    node = next;
                }
                None => return false,
            };
        }
        node.word
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(next) => {
                    node = next;
                }
                None => return false,
            };
        }
        true
    }
}

struct TrieNode {
    prefix: String,
    word: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new(prefix: String) -> Self {
        Self {
            prefix,
            word: false,
            children: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));
        trie.insert("app".to_string());
        trie.search("app".to_string());
    }
}
