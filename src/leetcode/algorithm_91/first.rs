pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        fn char_to_digit(c: char) -> u8 {
            ((c as u8) - 0x30) as u8
        }
        let digits: Vec<u8> = s.chars().map(char_to_digit).collect();
        if digits[0] == 0 {
            return 0;
        }
        let mut ways = vec![1, 1, 1];

        for (i, &d) in digits.iter().enumerate().skip(1) {
            let previous = digits[i - 1];
            if d == 0 {
                if previous == 1 || previous == 2 {
                    ways.rotate_right(1);
                    ways[0] = ways[2];
                } else {
                    return 0;
                }
                continue;
            }
            ways.rotate_right(1);
            if (previous == 1 || previous == 2) && (previous * 10 + d <= 26) {
                ways[0] = ways[1] + ways[2];
            } else {
                ways[0] = ways[1];
            }
        }
        ways[0]
    }
}
