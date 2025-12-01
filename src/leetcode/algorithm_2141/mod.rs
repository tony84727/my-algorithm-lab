pub struct Solution;

impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let batteries: Vec<i64> = batteries.into_iter().map(|x| x as i64).collect();
        Self::solve(n as usize, &batteries)
    }

    fn solve(n: usize, batteries: &[i64]) -> i64 {
        let mut left = 0;
        let mut right = batteries.iter().sum::<i64>() / n as i64;
        while left <= right {
            let mid = left + (right - left) / 2;
            if Self::try_length(n, batteries, mid) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        right
    }

    fn try_length(n: usize, batteries: &[i64], length: i64) -> bool {
        let availables: i64 = batteries.iter().map(|b| b.min(&length)).sum();
        let required = n as i64 * length;
        availables >= required
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(2, vec![3,3,3] => 4; "example 1")]
    #[test_case(2, vec![1,1,1,1] => 2; "example 2")]
    fn test_solution(n: i32, batteries: Vec<i32>) -> i64 {
        Solution::max_run_time(n, batteries)
    }
}
