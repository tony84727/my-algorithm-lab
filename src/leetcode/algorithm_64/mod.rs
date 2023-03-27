pub struct Solution;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if x == 0 && y == 0 {
                    continue;
                }
                let left = if x > 0 { grid[y][x - 1] } else { i32::MAX };
                let top = if y > 0 { grid[y - 1][x] } else { i32::MAX };
                grid[y][x] += left.min(top);
            }
        }
        *grid.last().and_then(|row| row.last()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![
                vec![1,3,1],
                vec![1,5,1],
                vec![4,2,1],
    ] => 7; "example 1")]
    #[test_case(vec![
                vec![1,2,3],
                vec![4,5,6],
        ] => 12; "example 2")]
    fn test_solution(grid: Vec<Vec<i32>>) -> i32 {
        Solution::min_path_sum(grid)
    }
}
