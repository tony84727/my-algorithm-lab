use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn sign(n: i32) -> i32 {
        if n >= 0 {
            1
        } else {
            -1
        }
    }
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let sign = if Self::sign(numerator) * Self::sign(denominator) > 0 {
            String::new()
        } else {
            String::from('-')
        };
        if numerator == 0 {
            return String::from("0");
        }
        let numerator = numerator as i64;
        let denominator = denominator as i64;
        let mut remain_history: HashMap<i64, usize> = HashMap::new();
        let mut remain = numerator;
        let mut digits: Vec<char> = Vec::new();
        let mut enter_decimal = false;
        while remain != 0 {
            if !digits.is_empty() && !enter_decimal {
                digits.push('.');
                enter_decimal = true;
            }
            if let Some(index) = remain_history.get(&remain) {
                digits.insert(*index, '(');
                digits.push(')');
                break;
            }
            let divide = (remain / denominator).abs();
            let digit = divide.to_string();
            for c in digit.chars() {
                digits.push(c);
            }
            if enter_decimal {
                remain_history.insert(remain, digits.len() - 1);
            }
            remain %= denominator;
            remain *= 10;
        }
        sign + &String::from_iter(digits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1, 2 => "0.5"; "example 1")]
    #[test_case(2, 1 => "2"; "example 2")]
    #[test_case(4, 333 => "0.(012)"; "example 3")]
    #[test_case(1, 17 => "0.(0588235294117647)"; "case 1")]
    #[test_case(22, 7 => "3.(142857)"; "case 2")]
    #[test_case(0, 3 => "0"; "case 3")]
    #[test_case(-50, 8 => "-6.25"; "case 4")]
    #[test_case(7, -12 => "-0.58(3)"; "case 5")]
    #[test_case(-1, -2147483648 => "0.0000000004656612873077392578125"; "case 6")]
    #[test_case(420, 226 => "1.(8584070796460176991150442477876106194690265486725663716814159292035398230088495575221238938053097345132743362831)"; "case 7")]
    fn test_solution(numerator: i32, denominator: i32) -> String {
        Solution::fraction_to_decimal(numerator, denominator)
    }
}
