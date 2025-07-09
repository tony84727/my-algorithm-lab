pub struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut dp = vec![vec![0; k + 1]; start_time.len() + 1];
        let mut max_gap = 0;
        for (i, (start, end)) in start_time.iter().zip(end_time.iter()).enumerate() {
            dp[i + 1][0] = *end;
            for j in 1..=k {
                let shift_left = (start - dp[i][j - 1]).max(0);
                dp[i + 1][j] = end - shift_left;
                let next_start = if i + 1 >= start_time.len() {
                    event_time
                } else {
                    start_time[i + 1]
                };
                let gap = next_start - dp[i + 1][j];
                max_gap = max_gap.max(gap);
            }
        }
        max_gap
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5, 1, vec![1,3], vec![2,5] => 2; "example 1")]
    #[test_case(10, 1, vec![0,2,9], vec![1,4,10] => 6; "example 2")]
    #[test_case(5, 2, vec![0,1,2,3,4], vec![1,2,3,4,5] => 0; "example 3")]
    fn test_solution(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        Solution::max_free_time(event_time, k, start_time, end_time)
    }
}
