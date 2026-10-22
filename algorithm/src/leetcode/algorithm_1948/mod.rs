use std::collections::HashMap;

pub struct Solution;

#[derive(Default, Debug)]
struct Trie {
    serial: String,
    children: HashMap<String, Trie>,
}

impl Trie {
    fn insert(&mut self, path: &[String]) {
        let mut current = self;
        for p in path.iter() {
            current = current.children.entry(p.clone()).or_default();
        }
    }

    fn calculate_frequency(&mut self, frequency: &mut HashMap<String, usize>) {
        if self.children.is_empty() {
            return;
        }
        let mut serialized = vec![];
        for (folder, child) in self.children.iter_mut() {
            child.calculate_frequency(frequency);
            let child_serialized = format!("{folder}({})", child.serial);
            serialized.push(child_serialized);
        }
        serialized.sort_unstable();
        self.serial = serialized.join("");
        *frequency.entry(self.serial.clone()).or_default() += 1;
    }

    fn gather_unique(
        &self,
        prefix: &mut Vec<String>,
        frequency: &HashMap<String, usize>,
        answers: &mut Vec<Vec<String>>,
    ) {
        if frequency.get(&self.serial).cloned().unwrap_or_default() > 1 {
            return;
        }
        if !prefix.is_empty() {
            answers.push(prefix.clone());
        }

        for (folder, child) in self.children.iter() {
            prefix.push(folder.clone());
            child.gather_unique(prefix, frequency, answers);
            prefix.pop();
        }
    }
}

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut trie = Trie::default();
        for p in paths.iter() {
            trie.insert(p);
        }
        let mut frequency = HashMap::new();
        trie.calculate_frequency(&mut frequency);
        println!("{trie:?}");
        let mut prefix = Vec::new();
        let mut answers = Vec::new();
        trie.gather_unique(&mut prefix, &frequency, &mut answers);
        answers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![["a"], ["c"], ["d"],["a","b"],["c","b"],["d","a"]] => vecvec![["d"], ["d","a"]]; "example 1")]
    fn test_solution(paths: Vec<Vec<&'static str>>) -> Vec<Vec<String>> {
        Solution::delete_duplicate_folder(
            paths
                .into_iter()
                .map(|x| x.into_iter().map(String::from).collect())
                .collect(),
        )
    }
}
