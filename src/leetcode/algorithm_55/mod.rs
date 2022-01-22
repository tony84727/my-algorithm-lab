pub mod flood;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,3,1,1,4] => true; "example 1")]
    #[test_case(vec![2,5,0,0] => true; "case 1")]
    fn test_flood_solution(nums: Vec<i32>) -> bool {
        flood::Solution::can_jump(nums)
    }
}
