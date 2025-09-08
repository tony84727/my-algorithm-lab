pub struct Solution;

impl Solution {
    pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut changes = 0;
        while a | b | c > 0 {
            if c & 1 == 1 {
                if (a & 1) | (b & 1) == 0 {
                    changes += 1;
                }
            } else {
                if a & 1 == 1 {
                    changes += 1;
                }
                if b & 1 == 1 {
                    changes += 1;
                }
            }
            a >>= 1;
            b >>= 1;
            c >>= 1;
        }
        changes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(2,6,5 => 3; "example 1")]
    #[test_case(4,2,7 => 1; "example 2")]
    #[test_case(8,3,5 => 3; "case 1")]
    fn test_solution(a: i32, b: i32, c: i32) -> i32 {
        Solution::min_flips(a, b, c)
    }
}
