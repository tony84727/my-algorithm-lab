use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let chars = target.chars().collect::<Vec<char>>();
        let dictionary_length = words[0].len();
        let words = Self::index_chars(words);
        let mut dp = vec![vec![None; chars.len()]; dictionary_length + 1];
        Self::ways(&mut dp, &words, &chars, 0, 0)
    }

    fn index_chars(words: Vec<String>) -> Vec<HashMap<char, Vec<usize>>> {
        words
            .into_iter()
            .map(|s| {
                let mut index = HashMap::<char, Vec<usize>>::new();
                for (i, c) in s.char_indices() {
                    index.entry(c).or_default().push(i);
                }
                index
            })
            .collect()
    }

    fn ways(
        dp: &mut Vec<Vec<Option<i32>>>,
        words: &Vec<HashMap<char, Vec<usize>>>,
        chars: &[char],
        i: usize,
        start: usize,
    ) -> i32 {
        if start >= chars.len() {
            return 1;
        }
        if let Some(answer) = dp[i][start] {
            return answer;
        }
        let head = chars[start];
        let mut ways = 0_i64;
        for w in words.iter() {
            if let Some(indexes) = w.get(&head) {
                for char_index in indexes.iter() {
                    if *char_index >= i {
                        ways += Self::ways(dp, words, chars, char_index + 1, start + 1) as i64;
                    }
                }
            }
        }
        let ways = (ways % 1000000007) as i32;
        dp[i][start] = Some(ways);
        ways
    }
}
