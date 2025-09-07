pub struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(n as usize);
        if n % 2 == 0 {
            for n in 0..(n / 2) {
                result.push(n + 1);
                result.push(-(n + 1));
            }
            return result;
        }
        for n in 0..n / 2 {
            result.push(n + 1);
            result.push(-(n + 1));
        }
        result.push(0);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5; "example 1")]
    #[test_case(3; "example 2")]
    #[test_case(1; "example 3")]
    #[test_case(4; "case 1")]
    fn test_solution(n: i32) {
        let generated = Solution::sum_zero(n);
        assert_eq!(n as usize, generated.len());
        assert_eq!(0, generated.into_iter().sum());
    }
}
