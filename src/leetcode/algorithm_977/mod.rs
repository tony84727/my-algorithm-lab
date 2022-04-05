pub mod brute;
pub mod merging;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![-4,-1,0,3,10] => vec![0,1,9,16,100]; "example 1")]
    fn test_brute_solution(nums: Vec<i32>) -> Vec<i32> {
        brute::Solution::sorted_squares(nums)
    }

    #[test_case(vec![-4,-1,0,3,10] => vec![0,1,9,16,100]; "example 1")]
    #[test_case(vec![1] => vec![1]; "case 1")]
    fn test_merging_solution(nums: Vec<i32>) -> Vec<i32> {
        merging::Solution::sorted_squares(nums)
    }
}
