use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_lhs(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut index = HashMap::new();
        for (i, k) in nums.iter().enumerate() {
            index.insert(*k, i);
        }
        let mut ans = 0;
        for (i, min) in nums.iter().enumerate() {
            let max = *min + 1;
            let Some(j) = index.get(&max) else {
                continue;
            };
            if i >= *j {
                continue;
            }
            let length = j - i + 1;
            if ans < length {
                ans = length;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,3,2,2,5,2,3,7] => 5; "example 1")]
    #[test_case(vec![1,2,3,4] => 2; "example 2")]
    #[test_case(vec![1,1,1,1] => 0; "example 3")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::find_lhs(nums)
    }
}
