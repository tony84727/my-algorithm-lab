use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut bag: HashMap<i32, usize> = HashMap::new();
        for n in nums.into_iter() {
            let mut m = n % value;
            if m < 0 {
                m += value;
            }
            *bag.entry(m).or_default() += 1;
        }
        for current in 0.. {
            let m = current % value;
            let count = bag.entry(m).or_default();
            if *count == 0 {
                return current;
            }
            *count -= 1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,-10,7,13,6,8], 5 => 4; "example 1")]
    #[test_case(vec![1,-10,7,13,6,8], 7 => 2; "example 2")]
    #[test_case(vec![0, -3], 4 => 2; "case 1")]
    fn test_solution(nums: Vec<i32>, value: i32) -> i32 {
        Solution::find_smallest_integer(nums, value)
    }
}
