use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let n = nums.len();
        let target_rem = nums.iter().map(|&x| x as i64).sum::<i64>() % p as i64;

        if target_rem == 0 {
            return 0;
        }

        let mut prefix_rem = 0;
        let mut map = HashMap::new();
        map.insert(0, -1);
        let mut min_len = n as i32;

        for (i, &num) in nums.iter().enumerate() {
            prefix_rem = (prefix_rem + num as i64) % p as i64;
            let complement_rem = (prefix_rem - target_rem + p as i64) % p as i64;
            if let Some(&j) = map.get(&complement_rem) {
                min_len = min_len.min(i as i32 - j);
            }
            map.insert(prefix_rem, i as i32);
        }

        if min_len >= n as i32 {
            -1
        } else {
            min_len
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,1,4,2], 6 => 1; "example 1")]
    #[test_case(vec![6,3,5,2], 9 => 2; "example 2")]
    #[test_case(vec![1,2,3], 3 => 0; "example 3")]
    fn test_solution(nums: Vec<i32>, p: i32) -> i32 {
        Solution::min_subarray(nums, p)
    }
}
