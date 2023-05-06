pub struct Solution;

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return if nums[0] * 2 <= target { 1 } else { 0 };
        }
        let mut modulo_power = vec![1; nums.len()];
        let modulo = 1000000007;
        for i in 1..nums.len() {
            modulo_power[i] = (modulo_power[i - 1] * 2) % modulo;
        }
        nums.sort_unstable();
        let mut count = 0;
        for (i, min) in nums.iter().enumerate() {
            let mut left = i;
            let mut right = nums.len() - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                if nums[mid] + min > target {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            let left = left as i32 - 1;
            if left >= i as i32 {
                count += modulo_power[(left - i as i32) as usize];
                count %= modulo;
            }
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,5,6,7], 9 => 4; "example 1")]
    #[test_case(vec![3,3,6,8], 10 => 6; "example 2")]
    #[test_case(vec![2,3,3,4,6,7], 12 => 61; "example 3")]
    #[test_case(vec![1], 1 => 0; "case 1")]
    #[test_case(vec![14,4,6,6,20,8,5,6,8,12,6,10,14,9,17,16,9,7,14,11,14,15,13,11,10,18,13,17,17,14,17,7,9,5,10,13,8,5,18,20,7,5,5,15,19,14], 22 => 272187084; "case 2")]
    #[test_case(vec![27,21,14,2,15,1,19,8,12,24,21,8,12,10,11,30,15,18,28,14,26,9,2,24,23,11,7,12,9,17,30,9,28,2,14,22,19,19,27,6,15,12,29,2,30,11,20,30,21,20,2,22,6,14,13,19,21,10,18,30,2,20,28,22], 31 => 688052206; "case 3")]
    fn test_solution(nums: Vec<i32>, target: i32) -> i32 {
        Solution::num_subseq(nums, target)
    }
}
