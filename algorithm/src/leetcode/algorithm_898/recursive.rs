use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let results = Self::sub_array(0, &arr);
        results.len() as i32
    }

    fn sub_array(current: i32, arr: &[i32]) -> HashSet<i32> {
        let Some(n) = arr.first() else {
            return HashSet::new();
        };
        let mut with = Self::sub_array(current | n, &arr[1..]);
        let without = Self::sub_array(0, &arr[1..]);
        with.extend(without);
        with.insert(current | n);
        with
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
