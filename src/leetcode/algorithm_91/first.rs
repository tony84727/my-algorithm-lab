pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        fn char_to_digit(c: char) -> u8 {
            ((c as u8) - 0x30) as u8
        }
        let digits: Vec<u8> = s.chars().map(char_to_digit).collect();
        fn count_to_digits(s: &[u8]) -> Option<i32> {
            let mut count = 0;
            if s.len() == 0 {
                return None;
            }
            if s.len() == 1 && s[0] == 0 {
                return None;
            }
            for (i, d) in s.iter().enumerate().take(s.len() - 1) {
                let next = s[i + 1];
                match d {
                    1 | 2 if next <= 6 => {
                        count += 1;
                    }
                    _ => (),
                }
                if *d == 0 && (i == 0 || (s[i - 1] != 1 && s[i - 1] != 2)) {
                    return None;
                }
            }
            Some(count)
        }
        count_to_digits(&digits).unwrap_or(0)
    }
}
