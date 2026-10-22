pub mod brute;
pub mod kadane;
pub mod sliding_window;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,3,-2,4] => 6; "example 1")]
    #[test_case(vec![-2,0, -1] => 0; "example 2")]
    #[test_case(vec![-1,2,3,4,-9,-8] => 1728; "case 1")]
    #[test_case(vec![-1,0, 2,3,4,-9,-8] => 1728; "case 2")]
    #[test_case(vec![0,2] => 2; "case 3")]
    #[test_case(vec![2, 0] => 2; "case 4")]
    #[test_case(vec![-2] => -2; "case 5")]
    #[test_case(vec![-3,-1,-1] => 3; "case 6")]
    #[test_case(vec![-2,3,-4] => 24; "case 7")]
    #[test_case(vec![-1,-2,-3,0] => 6; "case 8")]
    #[test_case(vec![-1,0, 2,3,-100, 4,-9,-8] => 21600; "case 9")]
    fn test_sliding_window(nums: Vec<i32>) -> i32 {
        sliding_window::Solution::max_product(nums)
    }

    #[test_case(vec![2,3,-2,4] => 6; "example 1")]
    #[test_case(vec![-2,0, -1] => 0; "example 2")]
    #[test_case(vec![-1,2,3,4,-9,-8] => 1728; "case 1")]
    #[test_case(vec![-1,0, 2,3,4,-9,-8] => 1728; "case 2")]
    #[test_case(vec![0,2] => 2; "case 3")]
    #[test_case(vec![2, 0] => 2; "case 4")]
    #[test_case(vec![-2] => -2; "case 5")]
    #[test_case(vec![-3,-1,-1] => 3; "case 6")]
    #[test_case(vec![-2,3,-4] => 24; "case 7")]
    #[test_case(vec![-1,-2,-3,0] => 6; "case 8")]
    #[test_case(vec![-1,0, 2,3,-100, 4,-9,-8] => 21600; "case 9")]
    fn test_brute(nums: Vec<i32>) -> i32 {
        brute::Solution::max_product(nums)
    }

    #[test_case(vec![2,3,-2,4] => 6; "example 1")]
    #[test_case(vec![-2,0, -1] => 0; "example 2")]
    #[test_case(vec![-1,2,3,4,-9,-8] => 1728; "case 1")]
    #[test_case(vec![-1,0, 2,3,4,-9,-8] => 1728; "case 2")]
    #[test_case(vec![0,2] => 2; "case 3")]
    #[test_case(vec![2, 0] => 2; "case 4")]
    #[test_case(vec![-2] => -2; "case 5")]
    #[test_case(vec![-3,-1,-1] => 3; "case 6")]
    #[test_case(vec![-2,3,-4] => 24; "case 7")]
    #[test_case(vec![-1,-2,-3,0] => 6; "case 8")]
    #[test_case(vec![-1,0, 2,3,-100, 4,-9,-8] => 21600; "case 9")]
    fn test_kadane(nums: Vec<i32>) -> i32 {
        kadane::Solution::max_product(nums)
    }
}
