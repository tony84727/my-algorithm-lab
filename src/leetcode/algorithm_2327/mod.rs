pub struct Solution;

impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        let m = 1000000007;
        let mut dp = vec![vec![0; forget as usize]; (n + 1) as usize];
        dp[0][0] = 1;
        let forget = forget as usize;
        let delay = delay as usize;
        let n = n as usize;
        for d in 1..n {
            for f in 1..forget {
                dp[d][f] = dp[d - 1][f - 1];
                if f >= delay {
                    dp[d][0] = (dp[d][0] + dp[d - 1][f - 1]) % m;
                }
            }
        }
        dp[n - 1].iter().fold(0, |a, b| (a + b) % m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(6,2,4 => 5; "example 1")]
    #[test_case(4,1,3 => 6; "example 2")]
    fn test_solution(n: i32, delay: i32, forget: i32) -> i32 {
        Solution::people_aware_of_secret(n, delay, forget)
    }
}
