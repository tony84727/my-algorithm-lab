pub struct Solution;

impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;
        while !nums.len() > 1 && !nums.is_sorted() {
            Self::merge(&mut nums);
            count += 1;
        }
        count
    }

    fn merge(nums: &mut Vec<i32>) {
        let mut min = i32::MAX;
        let mut to_merge = nums.len();
        for (i, &a) in nums.iter().enumerate().take(nums.len() - 1) {
            let current = a + nums[i + 1];
            if current < min {
                to_merge = i;
                min = current;
            }
        }
        nums.remove(to_merge);
        nums.remove(to_merge);
        nums.insert(to_merge, min);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![5,2,3,1] => 2; "example 1")]
    #[test_case(vec![1,2,2] => 0; "example 2")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::minimum_pair_removal(nums)
    }
}
