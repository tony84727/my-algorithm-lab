use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid.first().unwrap().len();
        let mut visited = HashSet::new();
        Self::dfs(&mut visited, &grid, m, n, 0, 0, 0).unwrap()
    }

    fn dfs(
        visited: &mut HashSet<(usize, usize)>,
        grid: &[Vec<i32>],
        m: usize,
        n: usize,
        r: usize,
        c: usize,
        current_max: i32,
    ) -> Option<i32> {
        if visited.contains(&(r, c)) {
            return None;
        }
        if r == m - 1 && c == n - 1 {
            return Some(current_max.max(grid[r][c]));
        }
        let max = grid[r][c].max(current_max);
        let mut min: Option<i32> = None;
        let mut add_min = |m: i32| {
            min = Some(match min {
                Some(current) => current.min(m),
                None => m,
            })
        };
        visited.insert((r, c));
        if r > 0 {
            if let Some(m) = Self::dfs(visited, grid, m, n, r - 1, c, max) {
                add_min(m);
            }
        }
        if r < m - 1 {
            if let Some(m) = Self::dfs(visited, grid, m, n, r + 1, c, max) {
                add_min(m);
            }
        }
        if c > 0 {
            if let Some(m) = Self::dfs(visited, grid, m, n, r, c - 1, max) {
                add_min(m);
            }
        }
        if c < n - 1 {
            if let Some(m) = Self::dfs(visited, grid, m, n, r, c + 1, max) {
                add_min(m);
            }
        }
        visited.remove(&(r, c));
        min.map(|m| m.max(max))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[0,2],[1,3]] => 3; "example 1")]
    #[test_case(vecvec![[0,1,2,3,4],[24,23,22,21,5],[12,13,14,15,16],[11,17,18,19,20],[10,9,8,7,6]] => 16; "example 2")]
    fn test_solution(grid: Vec<Vec<i32>>) -> i32 {
        Solution::swim_in_water(grid)
    }
}
