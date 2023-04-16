pub struct Solution;

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let chars = target.chars().collect::<Vec<char>>();
        let words: Vec<Vec<char>> = words.into_iter().map(|s| s.chars().collect()).collect();
        let mut dp = vec![vec![None; chars.len()]; words[0].len() + 1];
        Self::ways(&mut dp, &words, &chars, 0, 0)
    }

    fn ways(
        dp: &mut Vec<Vec<Option<i32>>>,
        words: &Vec<Vec<char>>,
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
            for (i, c) in w.iter().enumerate().skip(i) {
                if *c == head {
                    ways += Self::ways(dp, words, chars, i + 1, start + 1) as i64;
                }
            }
        }
        let ways = (ways % 1000000007) as i32;
        dp[i][start] = Some(ways);
        ways
    }
}
