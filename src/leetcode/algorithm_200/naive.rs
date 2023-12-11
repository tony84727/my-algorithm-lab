pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        use std::collections::HashMap;
        let row_lengh = grid.first().map(|f| f.len()).unwrap_or(0);
        struct Components {
            components: HashMap<usize, Vec<usize>>,
            lookup: HashMap<usize, usize>,
        }
        impl Components {
            fn ensure(&mut self, id: usize) {
                if self.lookup.contains_key(&id) {
                    return;
                }
                self.lookup.insert(id, id);
                self.components.insert(id, vec![id]);
            }

            fn replace_all(&mut self, from: usize, to: usize) {
                let out = self.components.remove(&from).unwrap();
                let entries = self.components.entry(to).or_default();
                for land in out.into_iter() {
                    self.lookup.insert(land, to);
                    entries.push(land);
                }
            }

            fn count(self) -> usize {
                self.components
                    .into_iter()
                    .filter(|(_, v)| !v.is_empty())
                    .count()
            }

            fn merge(&mut self, a: usize, b: usize) {
                self.ensure(a);
                self.ensure(b);
                let a = *self.lookup.get(&a).unwrap();
                let b = *self.lookup.get(&b).unwrap();
                let from = a.max(b);
                let to = a.min(b);
                self.replace_all(from, to);
            }
        }
        let mut component = Components {
            components: HashMap::new(),
            lookup: HashMap::new(),
        };
        for (x, row) in grid.iter().enumerate() {
            for (y, &land) in row.iter().enumerate() {
                if land == '0' {
                    continue;
                }
                let land_id = x * row_lengh + y;
                component.ensure(land_id);
                if x < grid.len() - 1 && grid[x + 1][y] == '1' {
                    component.merge(land_id, land_id + row_lengh);
                }
                if y < row.len() - 1 && grid[x][y + 1] == '1' {
                    component.merge(land_id, land_id + 1);
                }
                if x > 1 && grid[x - 1][y] == '1' {
                    component.merge(land_id, land_id - row_lengh);
                }
                if y > 1 && grid[x][y - 1] == '1' {
                    component.merge(land_id, land_id - 1);
                }
            }
        }
        component.count() as i32
    }
}
