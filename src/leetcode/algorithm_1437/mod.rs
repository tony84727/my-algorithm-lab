pub struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut last = -1;
        for (i, n) in nums.into_iter().enumerate() {
            let i = i as i32;
            if n == 1 {
                if last != -1 && i - last - 1 < k {
                    return false;
                }
                last = i;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,0,0,0,1,0,0,1], 2 => true; "example 1")]
    #[test_case(vec![1,0,0,1,0,1], 2 => false; "example 2")]
    fn test_solution(nums: Vec<i32>, k: i32) -> bool {
        Solution::k_length_apart(nums, k)
    }
}
