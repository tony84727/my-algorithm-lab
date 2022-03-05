use std::collections::{HashMap, HashSet};

pub struct Solution;

struct Scanning {
    to_find: HashMap<char, i32>,
    missing: HashSet<char>,
}

impl Scanning {
    fn new(target: String) -> Self {
        let mut to_find = HashMap::<char, i32>::new();
        let mut missing = HashSet::new();
        for c in target.chars() {
            *to_find.entry(c).or_default() += 1;
            missing.insert(c);
        }
        Self { to_find, missing }
    }
    fn meet(&mut self, c: char) {
        match self.to_find.get_mut(&c) {
            Some(count) => {
                *count -= 1;
                if *count <= 0 {
                    self.missing.remove(&c);
                }
            }
            None => (),
        }
    }

    fn remove(&mut self, c: char) {
        match self.to_find.get_mut(&c) {
            Some(count) => {
                *count += 1;
                if *count > 0 {
                    self.missing.insert(c);
                }
            }
            None => (),
        }
    }

    fn is_satisfied(&self) -> bool {
        return self.missing.is_empty();
    }
}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut scanning = Scanning::new(t);
        let mut right = 0;
        let mut answer: Option<&[char]> = None;
        let s: Vec<char> = s.chars().collect();
        for (left, first) in s.iter().enumerate() {
            while !scanning.is_satisfied() && right < s.len() {
                // try to shrink by moving left
                right += 1;
                scanning.meet(s[right - 1]);
            }
            scanning.remove(*first);
            if !scanning.is_satisfied() {
                match answer {
                    Some(current) if right - left < current.len() => {
                        answer = Some(&s[left..right]);
                    }
                    None => {
                        answer = Some(&s[left..right]);
                    }
                    _ => (),
                }
                if right >= s.len() {
                    break;
                }
            }
        }

        answer.unwrap_or_default().iter().collect()
    }
}
