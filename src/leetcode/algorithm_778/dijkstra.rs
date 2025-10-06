use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Eq, PartialEq)]
struct Vertex {
    distance: i32,
    r: usize,
    c: usize,
    version: usize,
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

pub struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid.first().unwrap().len();
        let mut distance_heap = BinaryHeap::new();
        let mut versions: HashMap<(usize, usize), usize> = HashMap::new();
        distance_heap.push(Vertex {
            distance: grid[0][0],
            r: 0,
            c: 0,
            version: 1,
        });
        versions.insert((0, 0), 1);
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        while let Some(Vertex {
            distance,
            r,
            c,
            version,
        }) = distance_heap.pop()
        {
            let coordinate = (r, c);
            if version != *versions.get(&coordinate).unwrap() {
                continue;
            }
            if visited.contains(&coordinate) {
                continue;
            }
            visited.insert(coordinate);
            if r == m - 1 && c == n - 1 {
                return distance;
            }
            for (dr, dc) in directions.into_iter() {
                let ir = (r as i32) + dr;
                let ic = (c as i32) + dc;
                if !(0..m as i32).contains(&ir)
                    || !(0..n as i32).contains(&ic)
                    || visited.contains(&(ir as usize, ic as usize))
                {
                    continue;
                }
                let alt = distance.max(grid[ir as usize][ic as usize]);
                let version = versions.entry((ir as usize, ic as usize)).or_default();
                *version += 1;
                distance_heap.push(Vertex {
                    distance: alt,
                    r: ir as usize,
                    c: ic as usize,
                    version: *version,
                });
            }
        }
        -1
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
