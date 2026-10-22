pub struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }
        let mut sum = 0;
        for i in 1..=n {
            sum += Solution::num_trees(i - 1) * Solution::num_trees(n - i);
        }
        sum
    }
}
