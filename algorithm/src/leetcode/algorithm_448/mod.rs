pub mod brute_force;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![4,3,2,7,8,2,3,1] => vec![5,6];"example 1")]
    fn test_brute_force(nums: Vec<i32>) -> Vec<i32> {
        brute_force::Solution::find_disappeared_numbers(nums)
    }
}
