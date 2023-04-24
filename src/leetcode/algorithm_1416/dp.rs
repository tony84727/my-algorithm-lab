pub struct Solution;

impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let klog = (k as f64).log10().ceil() as i32;
        let digits = s
            .chars()
            .map(|c| c.to_digit(10).map(|x| x as i32).unwrap())
            .collect::<Vec<i32>>();
        let mut dp = vec![None; digits.len()];
        Self::ways(&mut dp, &digits, 0, k, klog)
    }

    fn ways(
        dp: &mut Vec<Option<i32>>,
        digits: &[i32],
        head_start: usize,
        k: i32,
        klog: i32,
    ) -> i32 {
        if head_start == digits.len() {
            return 1;
        }
        if let Some(answer) = dp[head_start] {
            return answer;
        }
        let mut ways = 0_i64;
        for i in 0..(klog as usize) {
            if head_start + i + 1 > digits.len() {
                break;
            }
            if !Self::is_valid(&digits[head_start..head_start + i + 1], k) {
                continue;
            }
            ways += Self::ways(dp, digits, head_start + 1 + i, k, klog) as i64;
        }
        let answer = Self::mod_answer(ways);
        dp[head_start] = Some(answer);
        answer
    }

    fn mod_answer(ways: i64) -> i32 {
        (ways % 1000000007) as i32
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
