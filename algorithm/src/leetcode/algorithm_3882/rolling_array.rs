pub struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let c = grid.first().map(|row| row.len()).unwrap();
        let mut prev = vec![[false; 1024]; c];
        let mut current = vec![[false; 1024]; c];
        for (r, row) in grid.iter().enumerate() {
            std::mem::swap(&mut current, &mut prev);
            current.fill([false; 1024]);
            if r == 0 {
                current[0][grid[0][0] as usize] = true;
            }
            for (c, &cell) in row.iter().enumerate() {
                if r > 0 {
                    for (i, &possible) in prev[c].iter().enumerate() {
                        if possible {
                            current[c][i ^ cell as usize] = true;
                        }
                    }
                }
                if c > 0 {
                    for i in 0..1024 {
                        if current[c - 1][i] {
                            current[c][i ^ cell as usize] = true;
                        }
                    }
                }
            }
        }
        current[c - 1].into_iter().position(|x| x).unwrap() as i32
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
