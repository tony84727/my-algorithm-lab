pub struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for s in strs {
            let (zeros, ones) = Self::count_zero_one(&s);
            if zeros > m || ones > n {
                continue;
            }
            for i in (zeros..=m).rev() {
                for j in (ones..=n).rev() {
                    dp[i][j] = dp[i][j].max(dp[i - zeros][j - ones] + 1);
                }
            }
        }

        dp[m][n]
    }

    fn count_zero_one(s: &str) -> (usize, usize) {
        let mut zeros = 0;
        let mut ones = 0;
        for b in s.bytes() {
            if b == b'0' {
                zeros += 1;
            } else if b == b'1' {
                ones += 1;
            }
        }
        (zeros, ones)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec!["10","0001","111001","1","0"], 5, 3 => 4; "example 1")]
    #[test_case(vec!["10","0","1"], 1, 1 => 2; "example 2")]
    fn test_solution(strs: Vec<&str>, m: i32, n: i32) -> i32 {
        let strs = strs.into_iter().map(|s| s.to_string()).collect();
        Solution::find_max_form(strs, m, n)
    }
}
