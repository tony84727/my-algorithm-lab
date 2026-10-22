pub struct Solution;

impl Solution {
    pub fn hamming_weight(mut n: u32) -> i32 {
        let mut count = 0;
        for _ in 0..32 {
            if (n & 1) % 2 != 0 {
                count += 1;
            }
            n >>= 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(11 => 3; "example 1")]
    fn test_solution(n: u32) -> i32 {
        Solution::hamming_weight(n)
    }
}
