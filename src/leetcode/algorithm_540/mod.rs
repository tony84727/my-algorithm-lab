pub mod xor;

#[cfg(test)]
mod tests {
    use super::xor;
    use test_case::test_case;

    #[test_case(vec![1,1,2,3,3,4,4,8,8] => 2; "example 1")]
    fn test_xor_solution(nums: Vec<i32>) -> i32 {
        xor::Solution::single_non_duplicate(nums)
    }
}
