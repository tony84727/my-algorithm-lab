pub struct Solution;

impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut word = String::from("a");
        for o in operations.into_iter() {
            match o {
                0 => {
                    word = word.repeat(2);
                }
                1 => {
                    word += &Self::shift(&word);
                }
                _ => (),
            }
        }
        word.chars().nth(k as usize - 1).unwrap()
    }

    fn shift(word: &str) -> String {
        let mut shifted = String::new();
        for c in word.chars() {
            shifted.push(((c as u8 - b'a' + 1) % 26 + b'a') as char);
        }
        shifted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5, vec![0,0,0] => 'a'; "example 1")]
    #[test_case(10, vec![0,1,0,1] => 'b'; "example 2")]
    fn test_solution(k: i64, operations: Vec<i32>) -> char {
        Solution::kth_character(k, operations)
    }
}
