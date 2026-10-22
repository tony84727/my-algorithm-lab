pub struct Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut max: Option<u32> = None;
        let mut consecutive = 0;
        let mut last = None;
        for c in num.chars() {
            let digit = c.to_digit(10).unwrap();
            if Some(digit) == last {
                consecutive += 1;
            } else {
                consecutive = 1;
            }
            if consecutive == 3 {
                max = Some(max.unwrap_or_default().max(digit));
            }
            last = Some(digit);
        }
        max.map(|d| d.to_string().repeat(3)).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("6777133339" => "777"; "example 1")]
    #[test_case("2300019" => "000"; "example 2")]
    #[test_case("42352338" => ""; "example 3")]
    fn test_solution(num: &str) -> String {
        Solution::largest_good_integer(String::from(num))
    }
}
