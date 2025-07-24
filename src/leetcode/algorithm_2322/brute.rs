use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut min = i32::MAX;
        let edges: Vec<Vec<usize>> = edges
            .into_iter()
            .map(|x| x.into_iter().map(|x| x as usize).collect())
            .collect();
        for i in 0..edges.len() {
            for j in (i + 1)..edges.len() {
                let mut connected: Vec<usize> = Vec::with_capacity(nums.len());
                for i in 0..nums.len() {
                    connected.push(i);
                }
                let mut parent_scores = nums.clone();
                for (ei, e) in edges.iter().enumerate() {
                    if ei == i || ei == j {
                        continue;
                    }
                    let mut first = e[0];
                    let mut second = e[1];
                    if first > second {
                        (first, second) = (second, first);
                    }
                    let mut to_update = vec![first, second];
                    let mut first_root = first;
                    while connected[first_root] != first_root {
                        first_root = connected[first_root];
                        to_update.push(first_root);
                    }
                    let mut second_root = second;
                    while connected[second_root] != second_root {
                        second_root = connected[second_root];
                        to_update.push(second_root);
                    }
                    let new_root = first_root.min(first).min(second_root);
                    let current_score = parent_scores[first_root] ^ parent_scores[second_root];
                    parent_scores[new_root] = current_score;
                    for parent in connected.iter_mut() {
                        if [first, second, first_root, second_root].contains(parent) {
                            *parent = new_root;
                        }
                    }
                }
                let mut scores = vec![];
                let roots: HashSet<usize> = HashSet::from_iter(connected.clone().into_iter());
                for root in roots.into_iter() {
                    scores.push(parent_scores[root]);
                }
                scores.sort();
                min = min.min(scores[2] - scores[0]);
            }
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vec![1,5,5,4,11], vecvec![[0,1],[1,2],[1,3],[3,4]] => 9; "example 1")]
    #[test_case(vec![5,5,2,4,4,2], vecvec![[0,1],[1,2],[5,2],[4,3],[1,3]] => 0; "example 2")]
    #[test_case(vec![29,29,23,32,17], vecvec![[3,1],[2,3],[4,1],[0,4]] => 15; "case 1")]
    #[test_case(vec![18,4,5,27,22,18], vecvec![[5,4],[2,4],[2,0],[4,1],[3,2]] => 6; "case 2")]
    fn test_solution(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        Solution::minimum_score(nums, edges)
    }
}
