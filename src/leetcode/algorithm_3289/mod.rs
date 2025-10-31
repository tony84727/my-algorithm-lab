pub struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut duplicated = Vec::new();
        let mut last = -1;
        for &n in nums.iter() {
            if n == last {
                duplicated.push(n);
            }
            last = n;
        }
        duplicated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![0,1,1,0] => vec![0,1]; "example 1")]
    #[test_case(vec![0,3,2,1,3,2] => vec![2,3]; "example 2")]
    #[test_case(vec![7,1,5,4,3,4,6,0,9,5,8,2] => vec![4,5]; "example 3")]
    fn test_solution(nums: Vec<i32>) -> Vec<i32> {
        Solution::get_sneaky_numbers(nums)
    }
}
