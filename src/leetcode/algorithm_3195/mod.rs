pub struct Solution;

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut top_left = (i32::MAX, i32::MAX);
        let mut bottom_right = (0, 0);

        for (r, row) in grid.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell != 1 {
                    continue;
                }
                top_left.0 = top_left.0.min(r as i32);
                top_left.1 = top_left.1.min(c as i32);
                bottom_right.0 = bottom_right.0.max(r as i32);
                bottom_right.1 = bottom_right.1.max(c as i32);
            }
        }

        let height = (bottom_right.0 - top_left.0).abs() + 1;
        let width = (bottom_right.1 - top_left.1).abs() + 1;
        height * width
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[0,1,0], [1,0,1]] => 6; "example 1")]
    #[test_case(vecvec![[1,0],[0,0]] => 1; "example 2")]
    fn test_solution(grid: Vec<Vec<i32>>) -> i32 {
        Solution::minimum_area(grid)
    }
}
