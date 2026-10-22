pub struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut encoded = String::new();
        let ones = ["I", "X", "C", "M"];
        let middles = ["V", "L", "D", ""];
        let tens = ["X", "C", "M", ""];
        for i in (0..4).rev() {
            let n = num / 10_i32.pow(i);
            num %= 10_i32.pow(i);
            encoded += &Self::to_roman(n, ones[i as usize], middles[i as usize], tens[i as usize]);
        }
        encoded
    }

    fn to_roman(num: i32, one: &str, middle: &str, ten: &str) -> String {
        match num {
            1..=3 => one.repeat(num as usize),
            4 => format!("{one}{middle}"),
            5 => middle.to_owned(),
            6..=8 => format!("{middle}{}", one.repeat((num - 5) as usize)),
            9 => format!("{one}{ten}"),
            10 => ten.to_owned(),
            _ => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3, "I", "V", "X" => "III".to_string())]
    #[test_case(5, "I", "V", "X" => "V".to_string())]
    #[test_case(8, "I", "V", "X" => "VIII".to_string())]
    #[test_case(9, "I", "V", "X" => "IX".to_string())]
    fn test_to_roman(num: i32, one: &str, middle: &str, ten: &str) -> String {
        Solution::to_roman(num, one, middle, ten)
    }

    #[test_case(3 => "III".to_string(); "example 1")]
    #[test_case(58 => "LVIII".to_string(); "example 2")]
    #[test_case(1994 => "MCMXCIV".to_string(); "example 3")]
    fn test_solution(nums: i32) -> String {
        Solution::int_to_roman(nums)
    }
}
