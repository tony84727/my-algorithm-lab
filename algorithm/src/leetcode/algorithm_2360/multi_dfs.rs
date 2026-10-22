use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let mut current = (0..edges.len() as i32).collect::<Vec<i32>>();
        let mut visisted = vec![HashSet::<i32>::new(); edges.len()];
        let mut longest = -1;
        let mut steps = 0;
        loop {
            let mut advanced = false;
            for (start, n) in current.iter_mut().enumerate() {
                if *n == -1 {
                    continue;
                }
                if visisted[start].contains(n) {
                    if *n as usize == start {
                        longest = longest.max(steps);
                    }
                    *n = -1;
                    continue;
                }
                visisted[start].insert(*n);
                let next = edges[*n as usize];
                *n = next;
                advanced = true;
            }
            steps += 1;
            if !advanced {
                break;
            }
        }
        longest
    }
}
