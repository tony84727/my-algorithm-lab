use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub struct Solution;

impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let c = c as usize;
        let mut offline = vec![false; c];
        let mut parents = Vec::from_iter(0..c);
        let mut nodes: HashMap<usize, BinaryHeap<Reverse<usize>>> = HashMap::new();
        for n in 0..c {
            let mut set = BinaryHeap::new();
            set.push(Reverse(n));
            nodes.insert(n, set);
        }
        let find = |parents: &mut [usize], node: usize| {
            let mut to_update = vec![];
            let mut current = node;
            while parents[current] != current {
                to_update.push(current);
                current = parents[current];
            }
            for n in to_update.into_iter() {
                parents[n] = current;
            }
            current
        };
        let mut join = |parents: &mut [usize], a: usize, b: usize| {
            let new_parent = a.min(b);
            let from = a.max(b);
            if new_parent == from {
                return;
            }
            parents[from] = new_parent;
            let mut parent_nodes = nodes.remove(&new_parent).unwrap();
            let mut to_add = nodes.remove(&from).unwrap();
            parent_nodes.append(&mut to_add);
            nodes.insert(new_parent, parent_nodes);
        };
        for connection in connections.iter() {
            let a = find(&mut parents, connection[0] as usize - 1);
            let b = find(&mut parents, connection[1] as usize - 1);
            join(&mut parents, a, b)
        }
        let mut results = Vec::with_capacity(queries.len());
        for q in queries {
            let operation = q[0];
            let node = q[1] as usize - 1;
            match operation {
                1 => {
                    if !offline[node] {
                        results.push(q[1]);
                        continue;
                    }
                    let parent = find(&mut parents, node);
                    let mut report: i32 = -1;
                    let nodes = nodes.get_mut(&parent).unwrap();
                    while let Some(Reverse(x)) = nodes.pop() {
                        if !offline[x as usize] {
                            report = x as i32;
                            break;
                        }
                    }
                    if report != -1 {
                        nodes.push(Reverse(report as usize));
                        results.push(report + 1);
                    } else {
                        results.push(report);
                    }
                }
                2 => {
                    offline[node] = true;
                }
                _ => (),
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(5, vecvec![[1,2],[2,3],[3,4],[4,5]], vecvec![[1,3],[2,1],[1,1],[2,2],[1,2]] => vec![3,2,3]; "example 1")]
    #[test_case(3, Vec::<Vec<i32>>::new(), vecvec![[1,1],[2,1],[1,1]] => vec![1, -1]; "example 2")]
    #[test_case(3, vecvec![[3,2],[1,3],[2,1]], vec![vec![2,1]] => Vec::<i32>::new(); "case 1")]
    fn test_solution(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Solution::process_queries(c, connections, queries)
    }
}
