pub struct Solution;

impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        Self::solve(&grid).min(Self::solve(&Self::rotate(&grid)))
    }

    fn solve(grid: &[Vec<i32>]) -> i32 {
        let last_column = grid.first().unwrap().len() - 1;
        let last_row = grid.len() - 1;
        let mut min_area = ((last_column + 1) * (last_row + 1)) as i32;
        for r in 0..last_row {
            for c in 0..last_column {
                min_area = min_area
                    .min(
                        Self::ones_area(grid, 0, 0, r, last_column)
                            + Self::ones_area(grid, r + 1, 0, last_row, c)
                            + Self::ones_area(grid, r + 1, c + 1, last_row, last_column),
                    )
                    .min(
                        Self::ones_area(grid, 0, 0, r, c)
                            + Self::ones_area(grid, 0, c + 1, r, last_column)
                            + Self::ones_area(grid, r + 1, 0, last_row, last_column),
                    );
            }
        }
        for r in 0..last_row - 1 {
            for r1 in r + 1..last_row {
                min_area = min_area.min(
                    Self::ones_area(grid, 0, 0, r, last_column)
                        + Self::ones_area(grid, r + 1, 0, r1, last_column)
                        + Self::ones_area(grid, r1 + 1, 0, last_row, last_column),
                )
            }
        }
        min_area
    }

    fn rotate(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let columns = grid.first().unwrap().len();
        let mut rotated = vec![vec![0; rows]; columns];
        for (r, row) in grid.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                rotated[columns - 1 - c][r] = cell;
            }
        }
        rotated
    }

    fn ones_area(grid: &[Vec<i32>], top: usize, left: usize, bottom: usize, right: usize) -> i32 {
        let mut top_left = (usize::MAX, usize::MAX);
        let mut bottom_right = (0, 0);

        for (r, row) in grid.iter().enumerate().skip(top).take(bottom - top + 1) {
            for (c, &cell) in row.iter().enumerate().skip(left).take(right - left + 1) {
                if cell == 1 {
                    top_left.0 = top_left.0.min(r);
                    top_left.1 = top_left.1.min(c);
                    bottom_right.0 = bottom_right.0.max(r);
                    bottom_right.1 = bottom_right.1.max(c);
                }
            }
        }
        if top_left.0 <= bottom_right.0 {
            return ((bottom_right.0 - top_left.0 + 1) * (bottom_right.1 - top_left.1 + 1)) as i32;
        }
        i32::MAX / 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[1,0,1], [1,1,1]] => 5; "example 1")]
    #[test_case(vecvec![[1,0,1,0], [0,1,0,1]] => 5; "example 2")]
    fn test_solution(grid: Vec<Vec<i32>>) -> i32 {
        Solution::minimum_sum(grid)
    }
}
