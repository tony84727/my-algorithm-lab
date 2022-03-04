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
        let mut is_satisfied = false;

        for (left, first) in s.iter().enumerate() {
            while !is_satisfied && right < s.len() {
                // try to shrink by moving left
                right += 1;
                if t.contains_key(&s[right - 1]) {
                    // meet.insert(s[right]);
                    *meet.entry(s[right - 1]).or_default() += 1;
                    if satisfy(&t, &meet) {
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
                match t.get(first) {
                    Some(&demand) => {
                        let count = meet.get_mut(first).unwrap();
                        *count -= 1;
                        if *count < demand {
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
