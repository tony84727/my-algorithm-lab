pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let x = x.to_string().chars().collect::<Vec<char>>();
        for i in 0..x.len() / 2{
            let counterpart = x.len() - i - 1;
            if x[i] != x[counterpart] {
                return false
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(121 => true; "example 1")]
    #[test_case(-121 => false; "example 2")]
    #[test_case(10 => false; "example 3")]
    fn test_solution(input: i32) -> bool {
        Solution::is_palindrome(input)
    }
}