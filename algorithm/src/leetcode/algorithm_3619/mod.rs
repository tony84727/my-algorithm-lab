use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let rn = grid.len();
        let cn = grid.first().map(|x| x.len()).unwrap();
        let mut components = vec![0; rn * cn];
        for (i, c) in components.iter_mut().enumerate() {
            *c = i;
        }
        let to_key = |r: usize, c: usize| r * cn + c;
        let find_parent = |components: &mut [usize], r: usize, c: usize| {
            let mut current = to_key(r, c);
            let mut to_update = vec![];
            while components[current] != current {
                to_update.push(current);
                current = components[current];
            }
            for u in to_update.into_iter() {
                components[u] = current;
            }
            current
        };
        let connect = |components: &mut [usize], r1: usize, c1: usize, r2: usize, c2: usize| {
            let p1 = find_parent(components, r1, c1);
            let p2 = find_parent(components, r2, c2);
            components[p1.max(p2)] = p1.min(p2);
        };
        for (r, row) in grid.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    continue;
                }
                if r + 1 < rn && grid[r + 1][c] > 0 {
                    connect(&mut components, r, c, r + 1, c);
                }
                if c + 1 < cn && grid[r][c + 1] > 0 {
                    connect(&mut components, r, c, r, c + 1);
                }
            }
        }
        let mut sum: HashMap<usize, i64> = HashMap::new();
        for (r, row) in grid.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    continue;
                }
                let component = find_parent(&mut components, r, c);
                *sum.entry(component).or_default() += *cell as i64;
            }
        }
        sum.into_values().filter(|x| *x % k as i64 == 0).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![
        [0, 2, 1, 0, 0],
        [0, 5, 0, 0, 5],
        [0, 0, 1, 0, 0],
        [0, 1, 4, 7, 0],
        [0, 2, 0, 0, 8]
    ], 5 => 2; "example 1")]
    #[test_case(vecvec![
        [3, 0, 3, 0],
        [0, 3, 0, 3],
        [3, 0, 3, 0]
    ], 3 => 6; "example 2")]
    #[test_case(vecvec![
        [0,0,0],
        [0,0,1],
        [11,0,6],
        [0,10,2],
        [0,0,0],
        [8,0,0]
    ], 19 => 1; "case 1")]
    fn test_count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        Solution::count_islands(grid, k)
    }
}
