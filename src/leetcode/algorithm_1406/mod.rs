pub struct Solution;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut dp = vec![0; n + 1];
        for i in (0..n).rev() {
            dp[i] = stone_value[i] - dp[i + 1];
            if i + 2 <= n {
                dp[i] = dp[i].max(stone_value[i] + stone_value[i + 1] - dp[i + 2]);
            }
            if i + 3 <= n {
                dp[i] =
                    dp[i].max(stone_value[i] + stone_value[i + 1] + stone_value[i + 2] - dp[i + 3]);
            }
        }
        match dp[0].cmp(&0) {
            std::cmp::Ordering::Equal => "Tie",
            std::cmp::Ordering::Less => "Bob",
            std::cmp::Ordering::Greater => "Alice",
        }
        .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,7] => "Bob".to_string(); "example 1")]
    #[test_case(vec![1,2,3,6] => "Tie".to_string(); "example 2")]
    fn test_solution(stone_value: Vec<i32>) -> String {
        Solution::stone_game_iii(stone_value)
    }
}
