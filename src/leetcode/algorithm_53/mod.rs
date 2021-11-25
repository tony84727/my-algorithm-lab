pub mod brute_force;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![-2,1,-3,4,-1,2,1,-5,4] => 6; "example 1")]
    #[test_case(vec![5,4,-1,7,8] => 23; "case 1")]
    #[test_case(vec![1,2,3,4,5,6] => 21; "case 2")]
    #[test_case(vec![1,2,3,-4,5,6] => 13; "case 3")]
    #[test_case(vec![-1] => -1; "case 4")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        brute_force::Solution::max_sub_array(nums)
    }
}
