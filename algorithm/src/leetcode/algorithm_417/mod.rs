pub struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights.first().unwrap().len();
        let empty = vec![vec![false; n]; m];
        let mut upper = empty.clone();
        let mut bottom = empty.clone();
        {
            let mut start = Vec::new();
            for r in 0..m {
                start.push((r, 0_usize));
            }
            for c in 1..n {
                start.push((0, c));
            }
            Self::bfs(&heights, m, n, &mut upper, start);
        }
        {
            let mut start = Vec::new();
            for r in 0..m {
                start.push((r, n - 1));
            }
            for c in 0..(n - 1) {
                start.push((m - 1, c));
            }
            Self::bfs(&heights, m, n, &mut bottom, start);
        }
        let mut results = Vec::new();
        for r in 0..m {
            for c in 0..n {
                if upper[r][c] && bottom[r][c] {
                    results.push(vec![r as i32, c as i32]);
                }
            }
        }
        results
    }

    fn bfs(
        heights: &[Vec<i32>],
        m: usize,
        n: usize,
        marked: &mut [Vec<bool>],
        start: Vec<(usize, usize)>,
    ) {
        let mut todo = start;
        while !todo.is_empty() {
            let current = std::mem::take(&mut todo);
            for (r, c) in current.into_iter() {
                if marked[r][c] {
                    continue;
                }
                marked[r][c] = true;
                let height = heights[r][c];
                if r > 0 && heights[r - 1][c] >= height {
                    todo.push((r - 1, c));
                }
                if c > 0 && heights[r][c - 1] >= height {
                    todo.push((r, c - 1));
                }
                if r < m - 1 && heights[r + 1][c] >= height {
                    todo.push((r + 1, c));
                }
                if c < n - 1 && heights[r][c + 1] >= height {
                    todo.push((r, c + 1));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]] => vecvec![[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]; "example 1")]
    fn test_solution(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Solution::pacific_atlantic(heights)
    }
}
