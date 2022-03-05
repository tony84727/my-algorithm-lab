use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        fn satisfy(to_find: &HashMap<char, i32>) -> bool {
            for (_, &v) in to_find.iter() {
                if v > 0 {
                    return false;
                }
            }
            true
        }
        let mut right = 0;
        let mut answer: Option<&[char]> = None;
        let s: Vec<char> = s.chars().collect();
        let mut to_find = {
            let mut target = HashMap::<char, i32>::new();
            for c in t.chars() {
                *target.entry(c).or_default() += 1;
            }
            target
        };

        let mut is_satisfied = false;

        for (left, first) in s.iter().enumerate() {
            while !is_satisfied && right < s.len() {
                // try to shrink by moving left
                right += 1;
                if let Some(count) = to_find.get_mut(&s[right - 1]) {
                    *count -= 1;
                    if satisfy(&to_find) {
                        is_satisfied = true;
                        match answer {
                            Some(current) if right - left < current.len() => {
                                answer = Some(&s[left..right]);
                            }
                            None => {
                                answer = Some(&s[left..right]);
                            }
                            _ => (),
                        }
                    }
                }
            }
            if is_satisfied {
                match to_find.get_mut(first) {
                    Some(count) => {
                        *count += 1;
                        if *count > 0 {
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
                            is_satisfied = false;
                        }
                    }
                    None => (),
                }
            }
        }

        answer.unwrap_or_default().iter().collect()
    }
}
