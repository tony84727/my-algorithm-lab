pub struct Solution;

impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        let mut frequency = [0_usize; 10];
        for d in Self::to_digits(n) {
            frequency[d as usize] += 1;
        }
        let mut last = n;
        'next: for number in n + 1.. {
            for position in 0..=((number as f32).log10().ceil() as u32) {
                let previous = Self::get_position_digit(last, position);
                let current = Self::get_position_digit(number, position);
                if let Some(previous) = previous {
                    frequency[previous as usize] -= 1;
                }
                if let Some(current) = current {
                    frequency[current as usize] += 1;
                }
                if current.map(|x| x != 0).unwrap_or_default() {
                    break;
                }
            }
            last = number;
            for (d, &count) in frequency.iter().enumerate() {
                if count == 0 {
                    continue;
                }
                if d != count {
                    continue 'next;
                }
            }
            return number;
        }
        0
    }

    fn to_digits(mut n: i32) -> Vec<i32> {
        let mut digits = Vec::new();
        while n != 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits
    }
    fn get_position_digit(n: i32, position: u32) -> Option<i32> {
        if n < 10_i32.pow(position) {
            return None;
        }
        Some((n / 10_i32.pow(position)) % 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1 => 22; "example 1")]
    #[test_case(1000 => 1333; "example 2")]
    #[test_case(3000 => 3133; "example 3")]
    fn test_solution(n: i32) -> i32 {
        Solution::next_beautiful_number(n)
    }

    #[test]
    fn test_get_position_digit() {
        assert_eq!(Some(0), Solution::get_position_digit(100, 0));
        assert_eq!(Some(3), Solution::get_position_digit(123, 0));
        assert_eq!(Some(2), Solution::get_position_digit(123, 1));
        assert_eq!(Some(1), Solution::get_position_digit(123, 2));
        assert_eq!(None, Solution::get_position_digit(123, 3));
    }
}
