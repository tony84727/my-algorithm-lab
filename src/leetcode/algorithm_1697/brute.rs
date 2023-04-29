use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn distance_limited_paths_exist(
        _n: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut edges: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        for e in edge_list.into_iter() {
            let a = e[0];
            let b = e[1];
            let distance = e[2];
            edges.entry(a).or_default().push((distance, b));
            edges.entry(b).or_default().push((distance, a));
        }
        for list in edges.values_mut() {
            list.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        }
        queries
            .into_iter()
            .map(|query| {
                let from = query[0];
                let to = query[1];
                let limit = query[2];
                let mut edges = edges.clone();
                Self::dfs(&mut edges, limit, from, to)
            })
            .collect()
    }

    fn dfs(edges: &mut HashMap<i32, Vec<(i32, i32)>>, limit: i32, from: i32, to: i32) -> bool {
        let outs = match edges.remove(&from) {
            Some(outs) => outs,
            None => return false,
        };
        for (distance, next) in outs.into_iter() {
            if distance >= limit {
                break;
            }
            if next == to {
                return true;
            }
            if Self::dfs(edges, limit, next, to) {
                return true;
            }
        }
        false
    }
}
