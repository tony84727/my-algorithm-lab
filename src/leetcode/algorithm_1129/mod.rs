use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn index_edge(edges: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        let mut m: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in edges.into_iter() {
            m.entry(edge[0]).or_default().push(edge[1]);
        }
        m
    }
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut answer = vec![-1; n as usize];
        let mut red_edges = Self::index_edge(red_edges);
        let mut blue_edges = Self::index_edge(blue_edges);
        let mut to_visit: Vec<(bool, i32, i32)> = vec![(true, 0, 0), (false, 0, 0)];
        while !to_visit.is_empty() {
            let mut next = vec![];
            for &(next_red, current, path_len) in to_visit.iter() {
                let mut edges = &mut blue_edges;
                if next_red {
                    edges = &mut red_edges;
                }
                if answer[current as usize] == -1 {
                    answer[current as usize] = path_len;
                } else {
                    answer[current as usize] = answer[current as usize].min(path_len);
                }
                for node in edges.remove(&current).unwrap_or(vec![]).iter() {
                    next.push((!next_red, *node, path_len + 1));
                }
            }
            std::mem::swap(&mut next, &mut to_visit);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3, vec![vec![0,1],vec![1,2]], vec![] => vec![0,1,-1]; "example 1")]
    #[test_case(3, vec![vec![0,1]], vec![vec![2,1]] => vec![0,1,-1]; "example 2")]
    #[test_case(3, vec![vec![0,1]], vec![vec![1,2]] => vec![0,1,2]; "case 1")]
    #[test_case(3, vec![vec![0,1], vec![0,2]], vec![vec![1,0]] => vec![0,1,1])]
    fn test_solution(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
        Solution::shortest_alternating_paths(n, red_edges, blue_edges)
    }
}
