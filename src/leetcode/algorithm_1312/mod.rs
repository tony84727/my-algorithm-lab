pub struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut dp = vec![0; n];
        if n <= 1 {
            return 0;
        }
        if n == 2 {
            if s[0] == s[1] {
                return 0;
            } else {
                return 1;
            }
        }
        for i in (0..n - 2).rev() {
            let mut previous = 0;
            for j in i + 1..n {
                let temp = dp[j];
                if s[i] == s[j] {
                    dp[j] = previous
                } else {
                    dp[j] = dp[j].min(dp[j - 1]) + 1
                }
                previous = temp;
            }
        }
        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("zzazz" => 0; "example 1")]
    #[test_case("mbadm" => 2; "example 2")]
    #[test_case("leetcode" => 5; "example 3")]
    #[test_case("g" => 0; "case 1")]
    #[test_case("no" => 1; "case 2")]
    fn test_solution(s: &str) -> i32 {
        Solution::min_insertions(s.to_owned())
    }
}
