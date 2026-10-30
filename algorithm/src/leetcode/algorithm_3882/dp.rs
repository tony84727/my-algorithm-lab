pub struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let r = grid.len();
        let c = grid.first().map(|row| row.len()).unwrap();
        let mut dp = vec![vec![[false; 1024]; c]; r];
        dp[0][0][grid[0][0] as usize] = true;
        for (r, row) in grid.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if r > 0 {
                    let prev = dp[r - 1][c];
                    for (i, &possible) in prev.iter().enumerate() {
                        if possible {
                            dp[r][c][i ^ cell as usize] = true;
                        }
                    }
                }
                if c > 0 {
                    let prev = dp[r][c - 1];
                    for (i, &possible) in prev.iter().enumerate() {
                        if possible {
                            dp[r][c][i ^ cell as usize] = true;
                        }
                    }
                }
            }
        }
        dp[r - 1][c - 1].into_iter().position(|x| x).unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[1, 2], [3, 4]] => 6; "example 1")]
    #[test_case(vecvec![[6, 7], [5, 8]] => 9; "example 2")]
    #[test_case(vec![vec![2, 7, 5]] => 0; "example 3")]
    fn test_min_cost(grid: Vec<Vec<i32>>) -> i32 {
        Solution::min_cost(grid)
    }
}
