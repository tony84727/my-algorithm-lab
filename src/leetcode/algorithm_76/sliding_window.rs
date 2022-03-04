use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        fn satisfy(target: &HashMap<char, usize>, current: &HashMap<char, usize>) -> bool {
            for (k, v) in target.iter() {
                match current.get(k) {
                    None => {
                        return false;
                    }
                    Some(current_count) if current_count < v => {
                        return false;
                    }
                    _ => (),
                }
            }
            true
        }
        let mut right = 0;
        let mut answer: Option<&[char]> = None;
        let s: Vec<char> = s.chars().collect();
        let t = {
            let mut target = HashMap::<char, usize>::new();
            for c in t.chars() {
                *target.entry(c).or_default() += 1;
            }
            target
        };

        let mut meet = HashMap::<char, usize>::new();

        for (left, first) in s.iter().enumerate() {
            while !satisfy(&t, &meet) && right < s.len() {
                // try to shrink by moving left
                if t.contains_key(&s[right]) {
                    // meet.insert(s[right]);
                    *meet.entry(s[right]).or_default() += 1;
                }
                right += 1;
            }
            if satisfy(&t, &meet) {
                match answer {
                    Some(current) if right - left < current.len() => {
                        answer = Some(&s[left..right]);
                    }
                    None => {
                        answer = Some(&s[left..right]);
                    }
                    _ => (),
                }
                if let Some(count) = meet.get_mut(first) {
                    if *count == 1 {
                        meet.remove(first);
                    } else {
                        *count -= 1;
                    }
                }
            } else {
                break;
            }
        }

        answer.unwrap_or_default().iter().collect()
    }
}
