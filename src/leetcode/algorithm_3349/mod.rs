pub struct Solution;

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        if nums.len() < 2 * k {
            return false;
        }
        if k == 1 {
            return true;
        }
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
        if subarray_lengths.iter().any(|x| *x >= 2 * k) {
            return true;
        }
        if subarray_lengths.len() < 2 {
            return false;
        }
        let mut count = 0;
        for &length in subarray_lengths.iter() {
            if length >= k {
                count += 1;
            } else {
                count = 0;
            }
            if count == 2 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,5,7,8,9,2,3,4,3,1], 3 => true; "example 1")]
    #[test_case(vec![1,2,3,4,4,4,4,5,6,7], 5 => false; "example 2")]
    #[test_case(vec![-15,19], 1 => true; "case 1")]
    #[test_case(vec![-3,-19,-8,-16], 2 => false; "case 2")]
    #[test_case(vec![5,8,-2,-1], 2 => true; "case 3")]
    #[test_case(vec![-15,-13,4,7], 2 => true; "case 4")]
    fn test_solution(nums: Vec<i32>, k: i32) -> bool {
        Solution::has_increasing_subarrays(nums, k)
    }
}
