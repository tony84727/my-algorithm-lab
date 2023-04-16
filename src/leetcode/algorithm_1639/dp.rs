pub struct Solution;

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let chars = target.chars().collect::<Vec<char>>();
        let dictionary_length = words[0].len();
        let occurences = Self::chars_occurrences(words);
        let m = 1000000007;
        let mut dp = vec![vec![0; dictionary_length + 1]; chars.len() + 1];
        dp[0][0] = 1;
        for i in 0..target.len() + 1 {
            for j in 0..dictionary_length {
                if i < chars.len() {
                    let head = chars[i];
                    dp[i + 1][j + 1] += ((dp[i][j] as i64
                        * occurences[(head as u8 - 'a' as u8) as usize][j] as i64)
                        % m as i64) as i32;
                    dp[i + 1][j + 1] %= m;
                }
                dp[i][j + 1] += dp[i][j];
                dp[i][j + 1] %= m;
            }
        }
        dp[target.len()][dictionary_length]
    }

    fn chars_occurrences(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut occurrences = vec![vec![0; words[0].len()]; 26];
        for w in words.into_iter() {
            for (i, c) in w.char_indices() {
                occurrences[((c as u8) - ('a' as u8)) as usize][i] += 1;
            }
        }
        occurrences
    }
}
