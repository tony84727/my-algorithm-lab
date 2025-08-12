pub struct Solution;

impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        let modulo = 1000000007;
        let x = x as u32;
        let n = n as usize;
        // dp[sum][max_int]
        let mut dp = vec![vec![0; n + 1]; n + 1];
        dp[0][0] = 1;
        for sum in 0..=n {
            for m in 1..=n {
                let power = m.pow(x);
                if power + sum > n {
                    continue;
                }
                dp[sum + power][m] = dp[sum].iter().take(m).fold(0, |a, b| (a + b) % modulo);
            }
        }
        dp[n].iter().fold(0, |a, b| (a + b) % modulo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(10, 2 => 1; "example 1")]
    #[test_case(4, 1=> 2; "example 2")]
    #[test_case(67, 1 => 22250; "case 1")]
    #[test_case(213, 1 => 55852583; "case 2")]
    fn test_solution(n: i32, x: i32) -> i32 {
        Solution::number_of_ways(n, x)
    }
}
