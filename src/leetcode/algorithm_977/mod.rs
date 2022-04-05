pub mod brute;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![-4,-1,0,3,10] => vec![0,1,9,16,100]; "example 1")]
    fn test_solution(nums: Vec<i32>) -> Vec<i32> {
        brute::Solution::sorted_squares(nums)
    }
}
