pub struct Solution;

impl Solution {
    pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut i = 0;
        num.reverse();
        let mut carry = false;
        while k > 0 || carry {
            if i >= num.len() {
                num.push(0);
            }
            num[i] += (k % 10) + (if carry { 1 } else { 0 });
            carry = false;
            if num[i] >= 10 {
                num[i] %= 10;
                carry = true;
            }
            k /= 10;
            i += 1;
        }
        num.reverse();
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,0,0], 34 => vec![1,2,3,4]; "example 1")]
    #[test_case(vec![2,7,4], 181 => vec![4,5,5]; "example 2")]
    #[test_case(vec![2,1,5], 806 => vec![1,0,2,1]; "example 3")]
    #[test_case(vec![], 100 => vec![1,0,0]; "case 1")]
    #[test_case(vec![9,9,9,9,9,9,9,9,9,9], 1 => vec![1,0,0,0,0,0,0,0,0,0,0]; "case 2")]
    #[test_case(vec![1], 33 => vec![3,4]; "case 3")]
    fn test_solution(num: Vec<i32>, k: i32) -> Vec<i32> {
        Solution::add_to_array_form(num, k)
    }
}
