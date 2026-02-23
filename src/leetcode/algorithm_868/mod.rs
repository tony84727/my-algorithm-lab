pub struct Solution;

impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let mut max = 0;
        let mut start: Option<i32> = None;
        let mut i = 0;
        while n > 0 {
            if n & 1 != 0 {
                if let Some(j) = start {
                    max = max.max(i - j);
                }
                start = Some(i);
            }
            n >>= 1;
            i += 1;
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(22 => 2; "example 1")]
    #[test_case(8 => 0; "example 2")]
    #[test_case(5 => 2; "example 3")]
    #[test_case(6 => 1; "case 1")]
    fn test_solution(n: i32) -> i32 {
        Solution::binary_gap(n)
    }
}
