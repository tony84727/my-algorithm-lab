pub struct Solution;

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut dp = vec![vec![0; (k + 1) as usize]; piles.len() + 1];
        for pc in 1..=piles.len() {
            for i in 0..=(k as usize) {
                let mut sum = 0;
                for c in 0..=piles[pc - 1].len().min(i) {
                    if c > 0 {
                        sum += piles[pc - 1][c - 1];
                    }
                    dp[pc][i] = dp[pc][i].max(dp[pc - 1][i - c] + sum);
                }
            }
        }
        dp[piles.len()][k as usize]
    }
}
