use std::collections::{BTreeMap, BTreeSet};

pub struct Solution;

struct Scanning {
    to_find: BTreeMap<char, i32>,
    missing: BTreeSet<char>,
}

impl Scanning {
    fn new(target: String) -> Self {
        let mut to_find = BTreeMap::<char, i32>::new();
        let mut missing = BTreeSet::new();
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

    fn is_important(&self, c: char) -> bool {
        return self.to_find.contains_key(&c);
    }
}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut scanning = Scanning::new(t);
        let mut left = 0;
        let mut answer: Option<&[char]> = None;
        let s: Vec<char> = s.chars().collect();
        for (right, &new) in s.iter().enumerate() {
            if scanning.is_important(new) {
                scanning.meet(new);
                if scanning.is_satisfied() {
                    for current_left in left..right {
                        if scanning.is_important(s[current_left]) {
                            scanning.remove(s[current_left]);
                            if !scanning.is_satisfied() {
                                match answer {
                                    Some(current) if right - current_left < current.len() => {
                                        answer = Some(&s[current_left..=right]);
                                    }
                                    None => {
                                        answer = Some(&s[current_left..=right]);
                                    }
                                    _ => (),
                                }
                                left = current_left + 1;
                                break;
                            }
                        }
                    }
                }
            }
        }
        answer.unwrap_or_default().iter().collect()
    }
}
