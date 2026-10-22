use std::collections::HashMap;

pub struct Solution;

struct Scanning {
    to_find: HashMap<char, i32>,
    missing: u32,
}

impl Scanning {
    fn new(target: String) -> Self {
        let mut to_find = HashMap::<char, i32>::new();
        for c in target.chars() {
            *to_find.entry(c).or_default() += 1;
        }
        let missing = to_find.len() as u32;
        Self { to_find, missing }
    }
    fn meet(&mut self, c: char) {
        if let Some(count) = self.to_find.get_mut(&c) {
            if *count == 1 {
                self.missing -= 1;
            }
            *count -= 1;
        }
    }

    fn remove(&mut self, c: char) {
        if let Some(count) = self.to_find.get_mut(&c) {
            if *count == 0 {
                self.missing += 1;
            }
            *count += 1;
        }
    }

    fn is_satisfied(&self) -> bool {
        self.missing == 0
    }

    fn is_important(&self, c: char) -> bool {
        self.to_find.contains_key(&c)
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
                    for current_left in left..=right {
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
