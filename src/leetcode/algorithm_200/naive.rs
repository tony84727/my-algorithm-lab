pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        use std::collections::HashSet;
        use std::iter::FromIterator;
        let row_lengh = grid.first().map(|f| f.len()).unwrap_or(0);
        let mut components = vec![-1_i32; grid.len() * row_lengh];
        fn union(components: &mut [i32], old: usize, new: usize) {
            let old = if components[old] < 0 {
                components[old] = old as i32;
                old as i32
            } else {
                components[old]
            };
            let new = if components[new] < 0 {
                components[new] = new as i32;
                new as i32
            } else {
                components[new]
            };
            if new == old {
                return;
            }
            let to_repalce = old.max(new);
            let union_id = old.min(new);
            for n in components.iter_mut() {
                if *n == to_repalce {
                    *n = union_id;
                }
            }
        }
        for (x, row) in grid.iter().enumerate() {
            for (y, &land) in row.iter().enumerate() {
                if land == '0' {
                    continue;
                }
                let land_id = x * row_lengh + y;
                if components[land_id] < 0 {
                    components[land_id] = land_id as i32;
                }
                if x < grid.len() - 1 && grid[x + 1][y] == '1' {
                    union(&mut components, land_id, land_id + row_lengh);
                }
                if y < row.len() - 1 && grid[x][y + 1] == '1' {
                    union(&mut components, land_id, land_id + 1);
                }
                if x > 1 && grid[x - 1][y] == '1' {
                    union(&mut components, land_id, land_id - row_lengh);
                }
                if y > 1 && grid[x][y - 1] == '1' {
                    union(&mut components, land_id, land_id - 1);
                }
            }
        }
        HashSet::<i32>::from_iter(components.into_iter().filter(|&x| x >= 0)).len() as i32
    }
}
