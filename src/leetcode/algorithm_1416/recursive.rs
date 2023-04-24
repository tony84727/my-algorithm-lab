pub struct Solution;

impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let klog = (k as f64).log10().ceil() as i32;
        let digits = s
            .chars()
            .map(|c| c.to_digit(10).map(|x| x as i32).unwrap())
            .collect::<Vec<i32>>();
        Self::ways(&digits, k, klog)
    }

    fn ways(digits: &[i32], k: i32, klog: i32) -> i32 {
        if digits.is_empty() {
            return 1;
        }
        let mut ways = 0;
        for i in 0..(digits.len()).min(klog as usize) {
            let (head, last) = digits.split_at(i + 1);
            if !Self::is_valid(head, k) {
                continue;
            }
            ways += Self::ways(last, k, klog);
        }
        ways
    }

    fn is_valid(digits: &[i32], k: i32) -> bool {
        if digits.is_empty() {
            return true;
        }
        if digits[0] == 0 {
            return false;
        }
        digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, d)| d * 10_i32.pow(i as u32))
            .sum::<i32>()
            <= k
    }
}
