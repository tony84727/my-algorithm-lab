pub struct Solution;

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut current = nums[0];
        let mut subarray_lengths = vec![];
        let mut start = 0;
        for (i, &next) in nums.iter().enumerate().skip(1) {
            if next <= current {
                subarray_lengths.push(i - start);
                start = i;
            } else if i == nums.len() - 1 {
                subarray_lengths.push(i + 1 - start);
            }
            current = next;
        }
        let mut max = 1;
        let mut last = 0;
        for length in subarray_lengths.into_iter() {
            max = max.max(last.min(length)).max(length / 2);
            last = length;
        }
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,5,7,8,9,2,3,4,3,1] => 3; "example 1")]
    #[test_case(vec![1,2,3,4,4,4,4,5,6,7] => 2; "example 2")]
    #[test_case(vec![-15,-13,4,7] => 2; "case 1")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::max_increasing_subarrays(nums)
    }
}
