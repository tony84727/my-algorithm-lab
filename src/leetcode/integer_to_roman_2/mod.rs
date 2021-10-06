pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let thoushand = num / 1000;
        let hundred = num % 1000 / 100;
        let ten = num % 100 / 10;
        let n = num % 10;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3 => "III".to_string(); "example 1")]
    #[test_case(4 => "IV".to_string(); "example 2")]
    #[test_case(9 => "IX".to_string(); "example 3")]
    #[test_case(58 => "LVIII".to_string(); "example 4")]
    #[test_case(1994 => "MCMXCIV"; "example 5")]
    fn test_solution(num: i32) -> String {
        Solution::int_to_roman(num)
    }
}
