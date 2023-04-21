pub struct Solution;
impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut dp =
            vec![vec![vec![0; (min_profit + 1) as usize]; group.len() + 1]; (n + 1) as usize];
        for n in 1..=(n as usize) {
            for (i, cost) in group.iter().enumerate().skip(1) {
                let cost = *cost as usize;

                let profit = profit[i] as usize;
                for remaining in 0..=(min_profit as usize) {
                    dp[n][i][remaining] += dp[n][i - 1][remaining] + dp[n - 1][i - 1][remaining];
                    if n < cost {
                        continue;
                    }
                    let next_remaining = if profit >= remaining {
                        0
                    } else {
                        remaining - profit
                    };
                    dp[n][i][remaining] += dp[n - cost][i][next_remaining] + 1;
                }
            }
        }
        dp.iter().map(|s| s[group.len()][0]).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5, 3, vec![2,2], vec![2,3] => 2; "example 1")]
    #[test_case(10, 5, vec![2,3,5], vec![6,7,8] => 7; "example 2")]
    fn test_solution(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        Solution::profitable_schemes(n, min_profit, group, profit)
    }
}
