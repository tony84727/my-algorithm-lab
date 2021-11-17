use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut table = HashMap::new();
        fn solve(table: &mut HashMap<(i32, i32), i32>, m: i32, n: i32) -> i32 {
            match table.get(&(m, n)) {
                Some(answer) => *answer,
                None => {
                    if m == 1 || n == 1 {
                        table.insert((m, n), 1);
                        return 1;
                    }
                    let answer = solve(table, m - 1, n) + solve(table, m, n - 1);
                    table.insert((m, n), answer);
                    answer
                }
            }
        }
        solve(&mut table, m, n)
    }
}
