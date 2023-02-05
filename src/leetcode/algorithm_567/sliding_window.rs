use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut remaining = {
            let mut m = HashMap::<char, i32>::new();
            for c in s1.chars() {
                *m.entry(c).or_default() += 1;
            }
            m
        };
        let mut start = 0;
        let mut end = 0;
        let chars = s2.chars().collect::<Vec<char>>();
        while start < s2.len() {
            while end < s2.len() {
                match remaining.get_mut(&chars[end]) {
                    Some(count) if *count > 0 => {
                        *count -= 1;
                        end += 1;
                        if remaining.values().all(|&x| x == 0) {
                            return true;
                        }
                    }
                    Some(_) => break,
                    None => {
                        for to_remove in chars.iter().take(end).skip(start) {
                            *remaining.get_mut(to_remove).unwrap() += 1;
                        }
                        start = end;
                        end += 1;
                        break;
                    }
                }
            }
            if remaining.values().all(|&x| x == 0) {
                return true;
            }
            if let Some(count) = remaining.get_mut(&chars[start]) {
                *count += 1;
            }
            start += 1;
        }
        false
    }
}
