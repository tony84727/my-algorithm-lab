pub struct Solution;

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid.first().map(|row| row.len()).unwrap();
        let mut elements: Vec<i32> = grid.into_iter().flatten().collect();
        let mut shifted = vec![vec![0; m]; n];
        elements.rotate_right(k as usize % (m * n));
        for r in 0..n {
            for c in 0..m {
                shifted[r][c] = elements[r * m + c];
            }
        }
        shifted
    }
}

#[cfg(test)]
mod tests {
    use crate::vecvec;

    use super::*;
    use test_case::test_case;

    #[test_case(vecvec![[3,8,1,9],[19,7,2,5],[4,6,11,10],[12,0,21,13]], 4 => vecvec![[12,0,21,13],[3,8,1,9],[19,7,2,5],[4,6,11,10]]; "example 1")]
    #[test_case(vecvec![[1,2,3],[4,5,6],[7,8,9]], 9 => vecvec![[1,2,3],[4,5,6],[7,8,9]]; "example 2")]
    #[test_case(vec![vec![1]], 100 => vec![vec![1]]; "case 1")]
    fn test_solution(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        Solution::shift_grid(grid, k)
    }
}
