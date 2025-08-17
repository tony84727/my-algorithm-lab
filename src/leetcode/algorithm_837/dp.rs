pub struct Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k > n {
            return 1.0;
        }
        let mut dp = vec![vec![0.0; k as usize + 1]; n as usize + 1];
        for row in dp.iter_mut() {
            row[0] = 1.0;
        }
        for vn in 1..=(n as usize) {
            for vk in 1..=(k as usize) {
                // Unoptimized version 1
                // for r in 1..=max_pts as usize {
                //     if vk <= r {
                //         dp[vn][vk] += if vn < r { 0.0 } else { 1.0 } / max_pts as f64;
                //         continue;
                //     }
                //     if vn <= r {
                //         continue;
                //     }
                //     dp[vn][vk] += dp[vn - r][vk - r] / max_pts as f64;
                // }
                for r in 1..vk.min(vn).min(max_pts as usize + 1) {
                    dp[vn][vk] += dp[vn - r][vk - r] / max_pts as f64;
                }
                if vn.min(max_pts as usize) >= vk.max(1) {
                    dp[vn][vk] +=
                        (vn.min(max_pts as usize) + 1 - vk.max(1)) as f64 / max_pts as f64;
                }
            }
        }
        dp[n as usize][k as usize]
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
