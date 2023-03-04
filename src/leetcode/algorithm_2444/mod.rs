pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut min_count = 0;
        let mut max_count = 0;
        let mut right = 0;
        let mut count = 0;
        for taken in nums.iter() {
            while right < nums.len() && nums[right] >= min_k && nums[right] <= max_k {
                if nums[right] == min_k {
                    min_count += 1;
                }
                if nums[right] == max_k {
                    max_count += 1;
                }
                if min_count > 0 && max_count > 0 {
                    count += 1;
                }
                right += 1;
            }
            if min_count > 0 && max_count > 0 {
                count += 1;
            }
            if *taken == min_k {
                min_count -= 1;
            }
            if *taken == max_k {
                max_count -= 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,1,1,1],1,1 => 10; "example 2")]
    fn test_solution(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        Solution::count_subarrays(nums, min_k, max_k)
    }
}
