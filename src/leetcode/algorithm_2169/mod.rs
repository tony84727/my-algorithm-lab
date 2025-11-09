pub struct Solution;

impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        let mut a = num1;
        let mut b = num2;
        let mut steps = 0;
        while a != 0 && b != 0 {
            (a, b) = (a.max(b), a.min(b));
            steps += a / b;
            a %= b;
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(2,3 => 3; "example 1")]
    fn test_solution(num1: i32, num2: i32) -> i32 {
        Solution::count_operations(num1, num2)
    }
}
