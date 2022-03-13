pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut previous = 1;
        let mut ways = 1;
        let digits = s.into_bytes();
        let mut digits = digits.iter().map(|c| c - 0x30);
        let mut last = digits.next().unwrap();
        if last == 0 {
            return 0;
        }
        for d in digits {
            if d == 0 {
                if last > 0 && last <= 2 {
                    ways = previous;
                } else {
                    return 0;
                }
            } else {
                let temp = ways;
                if last > 0 && last * 10 + d <= 26 {
                    ways += previous;
                }
                previous = temp;
            }
            last = d;
        }
        ways
    }
}
