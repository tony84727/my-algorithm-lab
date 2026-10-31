pub struct Solution;

const M: i64 = 1_000_000_007;
impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        let r = grid.len();
        let c = grid.first().map(|row| row.len()).unwrap();
        let mut dp = vec![vec![[0_i64; 2]; c]; r];
        for (r, row) in grid.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if r > 0 {
                    let sum = if r - 1 == 0 && c == 0 {
                        1
                    } else {
                        if grid[r - 1][c] == 1 {
                            dp[r - 1][c][0]
                        } else {
                            dp[r - 1][c].iter().sum()
                        }
                    };
                    dp[r][c][0] = (dp[r][c][0] + sum) % M;
                }
                if c > 0 {
                    let sum = if r == 0 && c - 1 == 0 {
                        1
                    } else {
                        if grid[r][c - 1] == 1 {
                            dp[r][c - 1][1]
                        } else {
                            dp[r][c - 1].iter().sum()
                        }
                    };
                    dp[r][c][1] = (dp[r][c][1] + sum) % M;
                }
                if cell == 1 {
                    let counts = &mut dp[r][c];
                    (counts[0], counts[1]) = (counts[1], counts[0]);
                }
            }
        }
        let goal = dp[r - 1][c - 1];
        (goal.iter().sum::<i64>() % M) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[0,1,0],[0,0,1],[1,0,0]] => 5; "example 1")]
    #[test_case(vecvec![[0,0],[0,0]] => 2; "example 2")]
    #[test_case(vecvec![[0,1,1],[1,1,0]] => 1; "example 3")]
    #[test_case(vecvec![[0,1],[1,0]] => 2; "case 1")]
    #[test_case(vecvec![[0,0],[0,1],[0,0]] => 2; "case 2")]
    fn test_solution(grid: Vec<Vec<i32>>) -> i32 {
        Solution::unique_paths(grid)
    }
}
