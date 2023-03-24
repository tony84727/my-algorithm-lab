use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn min_reorder(_n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut index = HashMap::<i32, Vec<usize>>::new();
        for (i, n) in connections.iter().enumerate() {
            index.entry(n[0]).or_default().push(i);
            index.entry(n[1]).or_default().push(i);
        }
        let mut to_visit = vec![0];
        let mut count = 0;
        let mut connected = HashSet::new();
        while let Some(current) = to_visit.pop() {
            connected.insert(current);
            let roads = index.get(&current).unwrap();
            for &road_id in roads.iter() {
                let from = connections[road_id][0];
                let to = connections[road_id][1];
                let from_visited = connected.contains(&from);
                let to_visited = connected.contains(&to);
                if from_visited && !to_visited {
                    count += 1;
                    to_visit.push(to);
                }
                if !from_visited {
                    to_visit.push(from);
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(6, vec![vec![0,1], vec![1,3], vec![2,3], vec![4,0], vec![4,5]] => 3; "example 1")]
    #[test_case(3, vec![vec![1,0], vec![2,0]] => 0; "example 2")]
    fn test_solution(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        Solution::min_reorder(n, connections)
    }
}
