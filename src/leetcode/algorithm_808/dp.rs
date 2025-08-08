pub struct Solution;

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        let m = ((n + 24) / 25) as usize;
        let mut dp = vec![vec![0_f64; m + 1]; m + 1];
        dp[0][0] = 0.5;
        for a in dp.iter_mut().skip(1) {
            a[0] = 0.0;
        }
        for b in 1..=m {
            dp[0][b] = 1.0;
        }
        for i in 1..=m {
            for j in 1..=m {
                if i == 0 && j == 0 {
                    continue;
                }
                dp[i][j] = 0.25
                    * (dp[i.saturating_sub(4)][j]
                        + dp[i.saturating_sub(3)][j.saturating_sub(1)]
                        + dp[i.saturating_sub(2)][j.saturating_sub(2)]
                        + dp[i.saturating_sub(1)][j.saturating_sub(3)])
            }
        }
        dp[m][m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(50 => 0.625; "example 1")]
    #[test_case(100 => 0.71875; "example 2")]
    #[test_case(1 => 0.625; "case 1")]
    #[test_case(51 => 0.65625; "case 2")]
    fn test_solution(n: i32) -> f64 {
        Solution::soup_servings(n)
    }
}
