pub struct Solution;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut chars: Vec<char> = s.chars().collect();
        if x >= y {
            return x * Self::remove_ab(&mut chars) + y * Self::remove_ba(&mut chars);
        }
        y * Self::remove_ba(&mut chars) + x * Self::remove_ab(&mut chars)
    }

    fn remove_ab(chars: &mut Vec<char>) -> i32 {
        let before = chars.len();
        let mut buffer: Vec<char> = vec![];
        for &c in chars.iter() {
            buffer.push(c);
            if buffer.ends_with(&['a', 'b']) {
                for _ in 0..2 {
                    buffer.pop();
                }
            }
        }
        let after = buffer.len();
        *chars = buffer;
        (before - after) as i32 / 2
    }

    fn remove_ba(chars: &mut Vec<char>) -> i32 {
        let before = chars.len();
        let mut buffer: Vec<char> = vec![];
        for &c in chars.iter() {
            buffer.push(c);
            if buffer.ends_with(&['b', 'a']) {
                for _ in 0..2 {
                    buffer.pop();
                }
            }
        }
        let after = buffer.len();
        *chars = buffer;
        (before - after) as i32 / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("cdbcbbaaabab", 4, 5 => 19; "example 1")]
    #[test_case("aabbaaxybbaabb", 5, 4=> 20; "example 2")]
    fn test_solution(s: &'static str, x: i32, y: i32) -> i32 {
        Solution::maximum_gain(String::from(s), x, y)
    }
}
