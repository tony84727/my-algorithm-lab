pub struct Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut last = 0;
        let mut pointer = 0;
        while pointer < nums.len() {
            let current = nums[pointer];
            if last != current {
                nums.remove(pointer);
            } else {
                pointer += 1;
            }
            last = current;
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![4,3,2,7,8,2,3,1] => vec![2,3]; "example 1")]
    #[test_case(vec![1,1,2] => vec![1]; "example 2")]
    fn test_solution(nums: Vec<i32>) -> Vec<i32> {
        Solution::find_duplicates(nums)
    }
}
