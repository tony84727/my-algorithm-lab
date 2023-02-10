pub struct Solution;

impl Solution {
    pub fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut to_propagate = vec![];
        let n = grid.len();
        let mut sea = n * n;
        for (r, row) in grid.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    to_propagate.push((r, c));
                    sea -= 1;
                }
            }
        }
        if sea == 0 {
            return -1;
        }
        let mut step = 0;
        while !to_propagate.is_empty() {
            let mut next = vec![];
            for &(r, c) in to_propagate.iter() {
                if r > 0 && grid[r - 1][c] == 0 {
                    grid[r - 1][c] = 1;
                    next.push((r - 1, c));
                }
                if r < n - 1 && grid[r + 1][c] == 0 {
                    grid[r + 1][c] = 1;
                    next.push((r + 1, c));
                }
                if c > 0 && grid[r][c - 1] == 0 {
                    grid[r][c - 1] = 1;
                    next.push((r, c - 1));
                }
                if c < n - 1 && grid[r][c + 1] == 0 {
                    grid[r][c + 1] = 1;
                    next.push((r, c + 1));
                }
            }
            step += 1;
            std::mem::swap(&mut next, &mut to_propagate);
        }
        step - 1
    }
}
