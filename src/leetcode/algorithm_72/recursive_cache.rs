pub struct Solution;

impl Solution {
    fn distance(dp: &mut Vec<Vec<i32>>, mut from: Vec<char>, mut to: Vec<char>) -> i32 {
        if from == to {
            return 0;
        }
        let from_len = from.len();
        let to_len = to.len();
        if from.is_empty() {
            return to_len as i32;
        }
        if to.is_empty() {
            return from_len as i32;
        }
        let cached_ans = dp[from_len - 1][to_len - 1];
        if cached_ans >= 0 {
            return cached_ans;
        }
        let a = from.last();
        let b = to.last();
        if a == b {
            from.pop();
            to.pop();
            return Self::distance(dp, from, to);
        }
        let mut options = vec![];
        if let Some(_last) = b {
            // insert & replace
            let mut replaced = from.clone();
            replaced.pop();
            let mut to = to.clone();
            to.pop();
            // insert
            options.push(Self::distance(dp, from.clone(), to.clone()));
            options.push(Self::distance(dp, replaced, to));
        }
        // delete
        let mut deleted = from;
        deleted.pop();
        options.push(Self::distance(dp, deleted, to));
        let ans = options.into_iter().min().unwrap() + 1;
        dp[from_len - 1][to_len - 1] = ans;
        ans
    }
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.chars().collect::<Vec<char>>();
        let word2 = word2.chars().collect::<Vec<char>>();
        let mut dp = vec![vec![-1; word2.len()]; word1.len()];
        Self::distance(&mut dp, word1, word2)
    }
}
