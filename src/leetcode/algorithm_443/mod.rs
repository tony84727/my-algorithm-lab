pub struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.is_empty() {
            return 0;
        }
        let mut i = 1;
        let mut count_index = 1;
        let mut count = 1;
        let mut current = chars[0];
        while i < chars.len() {
            match chars[i] {
                x if x == current => {
                    count += 1;
                    i += 1;
                }
                _ => {
                    if count > 1 {
                        chars.splice(count_index..i, count.to_string().chars());
                        i = count_index + 1;
                    }
                    count = 0;
                    count_index = i + 1;
                    current = chars[i];
                }
            }
        }
        if count > 1 {
            chars.splice(count_index..i, count.to_string().chars());
        }
        chars.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec!['a','a','b','b','c','c','c'], vec!['a','2','b','2','c','3']; "example 1")]
    fn test_solution(mut chars: Vec<char>, expected: Vec<char>) {
        Solution::compress(&mut chars);
        assert_eq!(expected, chars);
    }
}
