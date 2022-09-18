pub struct Solution;
/**
 *           4
 */
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let max = (n as f32).log2().ceil() as i32 + 1;
        if k > 1 {
            k + (n - 2_i32.pow((k - 2) as u32))
        } else {
            n
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1,2 => 2; "example 1")]
    #[test_case(2,6 => 3; "example 2")]
    #[test_case(3,14 => 4; "example 3")]
    #[test_case(1,3 => 3; "case 1")]
    #[test_case(2, 1=> 1; "case 2")]
    #[test_case(1,4 => 4; "case 3")]
    #[test_case(2,7 => 4; "case 4")]
    fn test_solution(k: i32, n: i32) -> i32 {
        Solution::super_egg_drop(k, n)
    }
}
