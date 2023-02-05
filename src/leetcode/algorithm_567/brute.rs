use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let target = {
            let mut m = HashMap::<char, i32>::new();
            for c in s1.chars() {
                *m.entry(c).or_default() += 1;
            }
            m
        };
        'a: for i in 0..s2.len() {
            let mut remaining = target.clone();
            for end in s2.chars().skip(i) {
                match remaining.get_mut(&end) {
                    Some(count) => {
                        *count -= 1;
                    }
                    None => continue 'a,
                }
                if remaining.values().all(|&x| x == 0) {
                    return true;
                }
            }
        }
        false
    }
}
