pub struct Solution;

impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort_unstable();
        let mut index = vec![];
        for (i, n) in nums.into_iter().enumerate() {
            if n == target {
                index.push(i as i32);
            }
            if n > target {
                break;
            }
        }
        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1, 2, 5, 2, 3], 2 => vec![1, 2]; "example 1")]
    #[test_case(vec![1, 2, 5, 2, 3], 3 => vec![3]; "example 2")]
    #[test_case(vec![1, 2, 5, 2, 3], 5 => vec![4]; "example 3")]
    fn test_target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Solution::target_indices(nums, target)
    }
}
