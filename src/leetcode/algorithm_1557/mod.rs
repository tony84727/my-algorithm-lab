use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sets = HashSet::new();
        for edge in edges.into_iter() {
            sets.insert(edge[1]);
        }
        (0..n).filter(|x| !sets.contains(x)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(6, vec![vec![0,1],vec![0,2],vec![2,5],vec![3,4],vec![4,2]] => vec![0,3]; "example 1")]
    #[test_case(5, vec![vec![0,1],vec![2,1],vec![3,1],vec![1,4],vec![2,4]] => vec![0,2,3]; "example 2")]
    fn test_solution(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Solution::find_smallest_set_of_vertices(n, edges);
        result.sort_unstable();
        result
    }
}
