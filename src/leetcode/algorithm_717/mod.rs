pub struct Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let n = bits.len();
        let mut i = 0;
        while i < n {
            if bits[i] == 0 {
                if i == n - 1 {
                    return true;
                }
                i += 1;
                continue;
            }
            i += 2;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,0,0] => true; "example 1")]
    #[test_case(vec![1,1,1,0] => false; "example 2")]
    #[test_case(vec![1,1,1,1,1,0] => false; "case 1")]
    #[test_case(vec![1,1,1,1,0,0] => true; "case 2")]
    #[test_case(vec![0,1,0] => false; "case 3")]
    fn test_solution(bits: Vec<i32>) -> bool {
        Solution::is_one_bit_character(bits)
    }
}
