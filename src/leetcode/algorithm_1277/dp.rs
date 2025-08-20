pub struct Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let row_count = matrix.len();
        let column_count = matrix.first().unwrap().len();
        let mut dp: Vec<Vec<i32>> = matrix.clone();
        let mut count = 0;
        for row in 0..row_count {
            for column in 0..column_count {
                if dp[row][column] > 0 && row > 0 && column > 0 {
                    dp[row][column] += dp[row - 1][column]
                        .min(dp[row - 1][column - 1])
                        .min(dp[row][column - 1]);
                }
                count += dp[row][column];
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::vecvec;

    use super::*;
    use test_case::test_case;

    #[test_case(vecvec![[0,1,1,1], [1,1,1,1], [0,1,1,1]] => 15; "example 1")]
    #[test_case(vecvec![[1,0,1], [1,1,0], [1,1,0]] => 7; "example 2")]
    #[test_case(vecvec![[0,0,0],[0,1,0], [0,1,0], [1,1,1], [1,1,0]] => 8; "case 1")]
    fn test_solution(matrix: Vec<Vec<i32>>) -> i32 {
        Solution::count_squares(matrix)
    }
}
