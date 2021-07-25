use std::collections::LinkedList;

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut matching = LinkedList::new();
        for c in s.chars() {
            match c {
                '(' | '{' | '[' => {
                    matching.push_front(c);
                }
                ')' | '}' | ']' => match matching.pop_front() {
                    None => {
                        return false;
                    }
                    Some(last) => {
                        if Self::close_form(last) != c {
                            return false;
                        }
                    }
                },
                _ => {
                    panic!("unexpected character");
                }
            }
        }
        matching.is_empty()
    }
    fn close_form(c: char) -> char {
        match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            _ => panic!("unexpected character"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("()" => true; "example 1")]
    #[test_case("()[]{}" => true; "example 2")]
    #[test_case("(]" => false; "example 3")]
    fn test_solution(input: &str) -> bool {
        Solution::is_valid(input.to_string())
    }
}
