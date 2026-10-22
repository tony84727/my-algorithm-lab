pub struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let mut chunks = vec![];
        let mut current = 0;
        let mut right = 0;
        let mut max = i64::MIN;
        for left in 0..=n - k {
            while right - left < k {
                current += nums[right] as i64;
                right += 1;
            }
            max = max.max(current);
            chunks.push(current);
            current -= nums[left] as i64;
        }
        while chunks.len() > k {
            let mut next = vec![];
            let cc = chunks.len();
            for i in 0..cc - k {
                let sum = chunks[i] + chunks[i + k];
                max = max.max(sum);
                next.push(sum);
            }
            chunks = next;
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2], 1 => 3; "example 1")]
    #[test_case(vec![-1,-2,-3,-4,-5], 4 => -10; "example 2")]
    #[test_case(vec![-5,1,2,-3,4], 2 => 4; "example 3")]
    fn test_solution(nums: Vec<i32>, k: i32) -> i64 {
        Solution::max_subarray_sum(nums, k)
    }
}
