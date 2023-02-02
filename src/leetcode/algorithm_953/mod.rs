use std::{cmp::Ordering, collections::HashMap, iter::FromIterator};

pub struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let order = HashMap::from_iter(order.char_indices().map(|(i, c)| (c, i)));
        let mut last = &words[0];
        for w in words.iter().skip(1) {
            if Self::alien_cmp(&order, last, w) == Ordering::Greater {
                return false;
            }
            last = w;
        }
        true
    }

    fn alien_cmp(order: &HashMap<char, usize>, a: &str, b: &str) -> Ordering {
        let mut a_chars = a.chars();
        let mut b_chars = b.chars();
        loop {
            let a = a_chars.next();
            let b = b_chars.next();
            let result = match (a, b) {
                (None, None) => {
                    break;
                }
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (Some(a), Some(b)) => order[&a].cmp(&order[&b]),
            };
            if result != Ordering::Equal {
                return result;
            }
        }
        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec!["hello", "leetcode"], "hlabcdefgijkmnopqrstuvwxyz" => true; "example 1")]
    fn test_solution(words: Vec<&str>, order: &str) -> bool {
        Solution::is_alien_sorted(
            words.into_iter().map(|x| x.to_owned()).collect(),
            order.to_owned(),
        )
    }
}
