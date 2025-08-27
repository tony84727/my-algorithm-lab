pub struct Solution;

impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        for (r, row) in grid.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    for direction in 0..4 {
                        max = max.max(1 + Self::find_lognest_v(&grid, direction, r, c, 2, false))
                    }
                }
            }
        }
        max
    }

    fn find_lognest_v(
        grid: &[Vec<i32>],
        direction: usize,
        r: usize,
        c: usize,
        expected: i32,
        turned: bool,
    ) -> i32 {
        let m = grid.len();
        let n = grid.first().unwrap().len();
        let directions = [(-1, 1), (1, 1), (1, -1), (-1, -1)];
        let offset = directions[direction];
        let turned_direction = (direction + 1) % 4;
        let next_row = r as i32 + offset.0;
        let next_column = c as i32 + offset.1;
        let next_expected = if expected == 2 { 0 } else { 2 };
        if !(0..(m as i32)).contains(&next_row)
            || !(0..(n as i32)).contains(&next_column)
            || grid[next_row as usize][next_column as usize] != expected
        {
            return 0;
        }
        let next_row = next_row as usize;
        let next_column = next_column as usize;
        if turned {
            return 1 + Self::find_lognest_v(
                grid,
                direction,
                next_row,
                next_column,
                next_expected,
                turned,
            );
        }
        1 + Self::find_lognest_v(
            grid,
            turned_direction,
            next_row,
            next_column,
            next_expected,
            true,
        )
        .max(Self::find_lognest_v(
            grid,
            direction,
            next_row,
            next_column,
            next_expected,
            false,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[2,2,1,2,2],[2,0,2,2,0],[2,0,1,1,0],[1,0,2,2,2],[2,0,0,2,2]] => 5; "example 1")]
    #[test_case(vecvec![[2,2,2,2,2],[2,0,2,2,0],[2,0,1,1,0],[1,0,2,2,2],[2,0,0,2,2]] => 4; "example 2")]
    #[test_case(vecvec![[1,2,2,2,2],[2,2,2,2,0],[2,0,0,0,0],[0,0,2,2,2],[2,0,0,2,0]] => 5; "example 3")]
    #[test_case(vec![vec![1]] => 1; "example 4")]
    #[test_case(vecvec![[2, 2, 0, 2, 0, 2, 0],[1, 2, 2, 1, 0, 2, 0]] => 2; "case 1")]
    fn test_solution(grid: Vec<Vec<i32>>) -> i32 {
        Solution::len_of_v_diagonal(grid)
    }
}
