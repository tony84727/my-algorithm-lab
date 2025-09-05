pub struct Solution;

impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        for i in 0..=60 {
            let composition = num1 as i64 - num2 as i64 * i;
            if composition <= 0 {
                continue;
            }
            if composition.count_ones() > i as u32 {
                continue;
            }
            if composition < i {
                continue;
            }
            return i as i32;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3, -2 => 3; "example 1")]
    #[test_case(5, 7 => -1; "example 2")]
    #[test_case(112577768, -501662198 => 16; "case 1")]
    #[test_case(85, 42 => -1; "case 2")]
    fn test_solution(num1: i32, num2: i32) -> i32 {
        Solution::make_the_integer_zero(num1, num2)
    }
}
