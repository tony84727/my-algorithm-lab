pub struct Solution;

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let mut min = nums.clone();
        let mut current_min = *min.last().unwrap();
        for n in min.iter_mut().rev() {
            if *n < current_min {
                current_min = *n;
                continue;
            }
            *n = current_min;
        }
        let mut current_max = nums[0];
        for (i, n) in nums.into_iter().enumerate() {
            current_max = current_max.max(n);
            if current_max - min[i] <= k {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![5, 0, 1, 4], 3 => 3; "example 1")]
    #[test_case(vec![3, 2, 1], 1 => -1; "example 2")]
    #[test_case(vec![0], 0 => 0; "example 3")]
    fn examples(nums: Vec<i32>, k: i32) -> i32 {
        Solution::first_stable_index(nums, k)
    }
}
