use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let mut visited = vec![false; edges.len()];
        let mut longest = -1;
        while let Some(start) = visited
            .iter()
            .enumerate()
            .find_map(|(i, &x)| if x { None } else { Some(i) })
        {
            if edges[start] == -1 {
                visited[start] = true;
                continue;
            }
            let mut current = start as i32;
            let mut path: Vec<i32> = vec![];
            let mut path_set = HashSet::<i32>::new();
            while current != -1 && !visited[current as usize] {
                if path_set.contains(&current) {
                    let mut length = 1;
                    while let Some(head) = path.pop() {
                        visited[head as usize] = true;
                        if head == current {
                            longest = longest.max(length);
                            break;
                        }
                        length += 1;
                    }
                    break;
                }
                path_set.insert(current);
                path.push(current);
                current = edges[current as usize];
            }
            for n in path.into_iter() {
                visited[n as usize] = true;
            }
        }
        longest
    }
}
