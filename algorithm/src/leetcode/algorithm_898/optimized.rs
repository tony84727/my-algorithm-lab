use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut results: HashSet<i32> = HashSet::new();
        let mut current: HashSet<i32> = HashSet::new();
        for n in arr.into_iter() {
            let mut next: HashSet<i32> = HashSet::new();
            results.insert(n);
            next.insert(n);
            for c in current.iter() {
                next.insert(c | n);
                results.insert(c | n);
            }
            current = next;
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
