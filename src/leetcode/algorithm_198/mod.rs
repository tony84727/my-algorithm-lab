pub mod brute_force;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,1] => 4; "example 1")]
    #[test_case(vec![2,1,1,2] => 4; "case 1")]
    fn test_brute_force_solution(nums: Vec<i32>) -> i32 {
        brute_force::Solution::rob(nums)
    }
}
