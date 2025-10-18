pub struct Solution;

impl Solution {
    pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut current = i32::MIN;
        let mut distinct = 0;
        for n in nums.into_iter() {
            let min = (current + 1).max(n - k).min(n + k);
            if min != current {
                distinct += 1;
            }
            current = min;
        }
        distinct
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,2,3,3,4], 2 => 6; "example 1")]
    #[test_case(vec![4,4,4,4], 1 => 3; "example 2")]
    fn test_solution(nums: Vec<i32>, k: i32) -> i32 {
        Solution::max_distinct_elements(nums, k)
    }
}
