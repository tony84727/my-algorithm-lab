pub mod flood;
pub mod max_range;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,3,1,1,4] => true; "example 1")]
    #[test_case(vec![3,2,1,0,4] => false; "example 2")]
    #[test_case(vec![2,5,0,0] => true; "case 1")]
    #[test_case(vec![0] => true; "case 2")]
    #[test_case(vec![0,2,3] => false; "case 3")]
    #[test_case(vec![1,2,0,1] => true; "case 4")]
    fn test_flood_solution(nums: Vec<i32>) -> bool {
        flood::Solution::can_jump(nums)
    }

    #[test_case(vec![2,3,1,1,4] => true; "example 1")]
    #[test_case(vec![3,2,1,0,4] => false; "example 2")]
    #[test_case(vec![2,5,0,0] => true; "case 1")]
    #[test_case(vec![0] => true; "case 2")]
    #[test_case(vec![0,2,3] => false; "case 3")]
    #[test_case(vec![1,2,0,1] => true; "case 4")]
    #[test_case(vec![5,9,3,2,1,0,2,3,3,1,0,0] => true; "case 5"
		)]
    fn test_max_range_solution(nums: Vec<i32>) -> bool {
        max_range::Solution::can_jump(nums)
    }
}
