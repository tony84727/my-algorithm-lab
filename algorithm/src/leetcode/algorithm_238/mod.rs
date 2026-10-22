pub mod brute_force;
pub mod smart;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,4] => vec![24,12,8,6]; "example 1")]
    #[test_case(vec![-1,1,0,-3,3] => vec![0,0,9,0,0]; "example 2")]
    fn test_brute_force_solution(nums: Vec<i32>) -> Vec<i32> {
        brute_force::Solution::product_except_self(nums)
    }

    #[test_case(vec![1,2,3,4] => vec![24,12,8,6]; "example 1")]
    #[test_case(vec![-1,1,0,-3,3] => vec![0,0,9,0,0]; "example 2")]
    fn test_smart_solution(nums: Vec<i32>) -> Vec<i32> {
        smart::Solution::product_except_self(nums)
    }
}
