pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut paths = vec![1u32; n as usize];
        for _ in 1..m {
            for x in 1..(n as usize) {
                paths[x] += paths[x - 1];
            }
        }
        *paths.last().unwrap() as i32
    }
}
