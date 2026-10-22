pub struct Solution;

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut results = Vec::with_capacity(nums.len());
        let mut current = 0;
        for n in nums.iter() {
            current = (2 * current + n) % 5;
            results.push(current == 0);
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![0,1,1] => vec![true,false,false]; "example 1")]
    #[test_case(vec![1,1,1] => vec![false,false,false]; "example 2")]
    #[test_case(vec![0,1,1,1,1,1] => vec![true,false,false,false,true,false]; "case 1")]
    fn test_solution(nums: Vec<i32>) -> Vec<bool> {
        Solution::prefixes_div_by5(nums)
    }
}
