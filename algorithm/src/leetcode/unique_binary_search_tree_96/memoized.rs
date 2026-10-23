use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut memoized = HashMap::new();
        fn solve(n: i32, memoized: &mut HashMap<i32, i32>) -> i32 {
            if n <= 1 {
                return 1;
            }
            match memoized.get(&n) {
                Some(solution) => *solution,
                None => {
                    let mut sum = 0;
                    for i in 1..=n {
                        sum += solve(i - 1, memoized) * solve(n - i, memoized);
                    }
                    memoized.insert(n, sum);
                    sum
                }
            }
        }
        solve(n, &mut memoized)
    }
}
