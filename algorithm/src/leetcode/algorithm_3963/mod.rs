pub struct Solution;

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let m = m as usize;
        let n = n as usize;
        let mut grid = vec![vec!['#'; n]; m];
        for cell in grid[0].iter_mut() {
            *cell = '.';
        }
        for row in grid.iter_mut() {
            row[n - 1] = '.';
        }
        grid.into_iter().map(|x| x.into_iter().collect()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(2, 3; "example 1")]
    #[test_case(3, 3; "example 2")]
    #[test_case(1, 4; "example 3")]
    fn test_solution(m: i32, n: i32) {
        let grid = Solution::create_grid(m, n);
        let (m, n) = (m as usize, n as usize);
        assert_eq!(grid.len(), m);

        let mut paths = vec![0_u8; n];
        paths[0] = 1;
        for row in &grid {
            assert_eq!(row.len(), n);
            for (j, cell) in row.bytes().enumerate() {
                assert!(matches!(cell, b'.' | b'#'));
                if cell == b'#' {
                    paths[j] = 0;
                } else if j > 0 {
                    paths[j] = (paths[j] + paths[j - 1]).min(2);
                }
            }
        }
        assert_eq!(paths[n - 1], 1, "expected exactly one right/down path");
    }
}
