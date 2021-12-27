use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<i32, Vec<String>>= HashMap::new();
        for s in strs {
            let mut hash = 0;
            for c in s.chars() {
                hash |= 1 << (c as u8) - 97;
            }
            map.entry(hash).or_default().push(s)
        }
        map.into_iter().map(|(_, v)| v).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::common::test_utils::from_file;
    use std::iter::FromIterator;
    use std::path::PathBuf;
    use std::collections::HashSet;

    use super::*;
    use serde::Deserialize;
    use test_case::test_case;

    #[derive(Deserialize)]
    struct TestCase {
        input: Vec<String>,
        answer: Vec<Vec<String>>,
    }

    #[test_case("example1.ron")]
    #[test_case("case1.ron")]
    fn test_solution(str: &str) {
        let TestCase{input, answer} = from_file(PathBuf::new().join("src/leetcode/algorithm_49").join(str));
        assert_eq!(HashSet::<Vec<String>>::from_iter(answer.into_iter()), HashSet::from_iter(Solution::group_anagrams(input).into_iter()));
    }
}