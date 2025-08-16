pub struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut digits = vec![];
        let mut current = num;
        while current != 0 {
            let digit = current % 10;
            digits.push(digit);
            current /= 10;
        }
        let mut changed = None;
        for (i, d) in digits.iter().enumerate().rev() {
            if d == &6 {
                changed = Some(i);
                break;
            }
        }
        num + changed
            .map(|i| 3 * 10_i32.pow(i as u32))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(9669 => 9969; "example 1")]
    #[test_case(9996 => 9999; "example 2")]
    #[test_case(9999 => 9999; "example 3")]
    fn test_solution(num: i32) -> i32 {
        Solution::maximum69_number(num)
    }
}
