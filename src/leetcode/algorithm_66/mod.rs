pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        digits.reverse();
        let mut carry = false;
        digits[0] += 1;
        if digits[0] > 9 {
            carry = true;
            digits[0] = 0;
        }
        let n = digits.len() - 1;
        let mut d = 0;
        while d < n && carry {
            carry = false;
            d += 1;
            digits[d] += 1;
            if digits[d] > 9 {
                carry = true;
                digits[d] = 0;
            }
        }
        if carry {
            digits.push(1);
        }
        digits.reverse();
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3] => vec![1,2,4]; "example 1")]
    #[test_case(vec![4,3,2,1] => vec![4,3,2,2]; "example 2")]
    #[test_case(vec![9] => vec![1,0]; "example 3")]
    #[test_case(vec![9,9,9] => vec![1,0,0,0]; "case 1")]
    fn test_solution(digits: Vec<i32>) -> Vec<i32> {
        Solution::plus_one(digits)
    }
}
