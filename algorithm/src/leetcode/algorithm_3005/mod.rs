use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut frequency: HashMap<i32, usize> = HashMap::new();
        let mut max = 0;
        for n in nums.into_iter() {
            let count = frequency.entry(n).or_default();
            *count += 1;
            max = max.max(*count);
        }
        frequency.into_values().filter(|x| x == &max).sum::<usize>() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,2,3,1,4] => 4; "example 1")]
    #[test_case(vec![1,2,3,4,5] => 5; "example 2")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::max_frequency_elements(nums)
    }
}
