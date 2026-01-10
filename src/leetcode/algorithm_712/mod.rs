pub struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1: Vec<i32> = s1.as_bytes().into_iter().map(|&x| x as i32).collect();
        let s2: Vec<i32> = s2.as_bytes().into_iter().map(|&x| x as i32).collect();
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        dp[0][0] = 0;
        let sum: i32 = s1.iter().sum::<i32>() + s2.iter().sum::<i32>();
        for (i, a) in s1.iter().enumerate() {
            for (j, b) in s2.iter().enumerate() {
                dp[i + 1][j + 1] = if a == b {
                    dp[i][j] + *a
                } else {
                    dp[i][j + 1].max(dp[i + 1][j])
                };
            }
        }
        sum - 2 * dp[s1.len()][s2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("sea", "eat" => 231; "example 1")]
    #[test_case("delete", "leet" => 403; "example 2")]
    fn test_solution(s1: &str, s2: &str) -> i32 {
        Solution::minimum_delete_sum(String::from(s1), String::from(s2))
    }
}
