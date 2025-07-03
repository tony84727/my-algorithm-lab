pub struct Solution;

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut word = String::from("a");
        for _ in 0..((k as f32).log2().ceil() as usize) {
            word += &Self::shift(&word);
            println!("{word}");
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

    #[test_case(5 => 'b'; "example 1")]
    #[test_case(10 => 'c'; "example 2")]
    fn test_solution(k: i32) -> char {
        Solution::kth_character(k)
    }
}
