pub struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let mut last_visited = vec![-1; edges.len()];
        let mut longest = -1;
        let mut tick = 0;
        for start in 0..edges.len() {
            tick += 1;
            last_visited[start] = tick;
            let start_tick = tick;
            let mut next = edges[start];
            while next != -1 && last_visited[next as usize] == -1 {
                tick += 1;
                last_visited[next as usize] = tick;
                next = edges[next as usize];
            }
            if next != -1 && last_visited[next as usize] >= start_tick {
                longest = longest.max(tick - last_visited[next as usize] + 1);
            }
        }
        longest
    }
}
