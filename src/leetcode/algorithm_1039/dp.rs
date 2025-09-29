pub struct Solution;

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut dp = vec![vec![-1; n]; n];
        Self::min_weight(&values, &mut dp, 0, n - 1)
    }

    fn min_weight(values: &[i32], dp: &mut [Vec<i32>], start: usize, end: usize) -> i32 {
        if dp[start][end] >= 0 {
            return dp[start][end];
        }
        if end - start == 2 {
            return values[start..=end].iter().product();
        }
        if end - start < 2 {
            return 0;
        }
        let mut min = i32::MAX;
        for k in start + 1..end {
            min = min.min(
                values[start] * values[k] * values[end]
                    + Self::min_weight(values, dp, start, k)
                    + Self::min_weight(values, dp, k, end),
            )
        }
        dp[start][end] = min;
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3] => 6; "example 1")]
    #[test_case(vec![3,7,4,5] => 144; "example 2")]
    #[test_case(vec![1,3,1,4,1,5] => 13; "example 3")]
    #[test_case(vec![1,2,3,4] => 18; "case 1")]
    #[test_case(vec![2,1,4,4] => 24; "case 2")]
    #[test_case(vec![69,22,21,27,26,62,69,81,55,85,95,40,91,33,72,88,86] => 1334781; "case 3")]
    #[test_case(vec![35,73,90,27,71,80,21,33,33,13,48,12,68,70,80,36,66,3,70,58] => 140295; "case 4")]
    fn test_solution(vertex: Vec<i32>) -> i32 {
        Solution::min_score_triangulation(vertex)
    }
}
