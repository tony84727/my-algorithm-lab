pub struct Solution;

impl Solution {
    fn distance(mut from: Vec<char>, mut to: Vec<char>) -> i32 {
        if from == to {
            return 0;
        }
        if from.is_empty() {
            return to.len() as i32;
        }
        if to.is_empty() {
            return from.len() as i32;
        }
        let a = from.last();
        let b = to.last();
        if a == b {
            from.pop();
            to.pop();
            return Self::distance(from, to);
        }
        let mut options = vec![];
        if let Some(_last) = b {
            // insert & replace
            let mut replaced = from.clone();
            replaced.pop();
            let mut to = to.clone();
            to.pop();
            // insert
            options.push(Self::distance(from.clone(), to.clone()));
            options.push(Self::distance(replaced, to));
        }
        // delete
        let mut deleted = from;
        deleted.pop();
        options.push(Self::distance(deleted, to));
        options.into_iter().min().unwrap() + 1
    }
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.chars().collect::<Vec<char>>();
        let word2 = word2.chars().collect::<Vec<char>>();
        Self::distance(word1, word2)
    }
}
