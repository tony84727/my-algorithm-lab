pub struct Solution;

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let swap = |grid: &mut Vec<Vec<i32>>, r1: usize, c1: usize, r2: usize, c2: usize| {
            let (first, second) = grid.split_at_mut(r2);
            std::mem::swap(&mut second[0][c2], &mut first[r1][c1]);
        };
        let n = grid.len();
        for start_at in 0..n {
            // bottom-left half: non-increasing
            for i in 1..(n - start_at) {
                for j in (0..i).rev() {
                    if grid[start_at + j][j] < grid[start_at + j + 1][j + 1] {
                        swap(&mut grid, start_at + j, j, start_at + j + 1, j + 1);
                    }
                }
            }
            // top-right half: non-decrasing
            if start_at > 0 {
                for i in 1..(n - start_at) {
                    for j in (0..i).rev() {
                        if grid[j][start_at + j] > grid[j + 1][start_at + j + 1] {
                            swap(&mut grid, j, start_at + j, j + 1, start_at + j + 1)
                        }
                    }
                }
            }
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[1,7,3],[9,8,2],[4,5,6]] => vecvec![[8,2,3],[9,6,7],[4,5,1]]; "example 1")]
    #[test_case(vecvec![[0,1],[1,2]] => vecvec![[2,1],[1,0]]; "example 2")]
    #[test_case(vec![vec![1]] => vec![vec![1]]; "example 3")]
    #[test_case(vecvec![[-1,-2,-3],[-3,-3,-2],[-4,-4,0]] => vecvec![[0,-2,-3],[-3,-1,-2],[-4,-4,-3]]; "case 1")]
    #[test_case(vecvec![[-1,2,1],[0,3,-5],[0,2,1]] => vecvec![[3,-5,1],[2,1,2],[0,0,-1]]; "case 2")]
    fn test_solution(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Solution::sort_matrix(grid)
    }
}
