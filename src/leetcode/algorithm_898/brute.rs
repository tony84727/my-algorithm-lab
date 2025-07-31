use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut results = HashSet::new();
        for start in 0..arr.len() {
            let mut current = 0;
            for end in arr.iter().skip(start) {
                current |= end;
                results.insert(current);
            }
        }
        results.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![0] => 1; "example 1")]
    #[test_case(vec![1,1,2] => 3; "example 2")]
    #[test_case(vec![1,2,4] => 6; "example 3")]
    fn test_solution(arr: Vec<i32>) -> i32 {
        Solution::subarray_bitwise_o_rs(arr)
    }
}
