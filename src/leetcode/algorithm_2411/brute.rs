pub struct Solution;

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let mut maximum: Vec<i32> = nums.clone();
        for (i, n) in nums.iter().enumerate().rev().skip(1) {
            maximum[i] = n | maximum[i + 1];
        }
        let mut count = vec![];
        for (i, _start) in nums.iter().enumerate() {
            let mut right = i;
            let mut current = 0;
            while right < nums.len() {
                current |= nums[right];
                right += 1;
                if current ^ maximum[i] == 0 {
                    break;
                }
            }
            count.push((right - i) as i32);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,0,2,1,3] => vec![3,3,2,2,1]; "example 1")]
    #[test_case(vec![1,2] => vec![2,1]; "example 2")]
    fn test_solution(nums: Vec<i32>) -> Vec<i32> {
        Solution::smallest_subarrays(nums)
    }
}
