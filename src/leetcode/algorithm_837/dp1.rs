pub struct Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        let mut dp = vec![0.0; (n as usize) + 1];
        dp[0] = 1.0;
        for x in 1..=(n as usize) {
            for r in (x.saturating_sub(k as usize) + 1)..=(max_pts as usize).min(x) {
                dp[x] += dp[x - r] / max_pts as f64;
            }
        }
        dp.into_iter().skip(k as usize).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(10, 1, 10 => 1.0; "example 1")]
    #[test_case(6, 1, 10 => 0.6; "example 2")]
    #[test_case(21, 17, 10 => 0.73278; "example 3")]
    #[test_case(1, 1, 2 => 0.5; "case 1")]
    fn test_solution(n: i32, k: i32, max_pts: i32) -> f64 {
        (Solution::new21_game(n, k, max_pts) * 100000.0).round() / 100000.0
    }
}
