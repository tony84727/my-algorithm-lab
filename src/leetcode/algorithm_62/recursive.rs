pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }
        return Solution::unique_paths(m - 1, n) + Solution::unique_paths(m, n - 1);
    }
}
