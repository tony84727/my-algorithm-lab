pub struct Solution;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for a in 1..n {
            if Self::has_zero(a) {
                continue;
            }
            if Self::has_zero(n - a) {
                continue;
            }
            return vec![a, n - a];
        }
        Vec::new()
    }

    fn has_zero(mut i: i32) -> bool {
        while i != 0 {
            if i % 10 == 0 {
                return true;
            }
            i /= 10;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(2 => vec![1,1]; "example 1")]
    #[test_case(11 => vec![2,9]; "example 2")]
    fn test_solution(n: i32) -> Vec<i32> {
        Solution::get_no_zero_integers(n)
    }
}
