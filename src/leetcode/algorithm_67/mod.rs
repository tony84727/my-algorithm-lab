pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a_bits = a
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i8)
            .into_iter()
            .rev();
        let mut b_bits = b
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i8)
            .into_iter()
            .rev();
        let mut out = String::new();
        let mut carry = 0;
        loop {
            let a = a_bits.next();
            let b = b_bits.next();
            if a.is_none() && b.is_none() && carry == 0 {
                break;
            }
            let a = a.unwrap_or(0);
            let b = b.unwrap_or(0);
            out.push(if a ^ b ^ carry > 0 { '1' } else { '0' });

            carry = (a & b) | (carry * (a ^ b));
        }
        out.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("11", "1" => "100"; "example 1")]
    #[test_case("1010", "1011" => "10101"; "example 2")]
    #[test_case("0", "0" => "0"; "case 1")]
    #[test_case("100", "110010" => "110110"; "case 2")]
    fn test_solution(a: &str, b: &str) -> String {
        Solution::add_binary(a.to_owned(), b.to_owned())
    }
}
