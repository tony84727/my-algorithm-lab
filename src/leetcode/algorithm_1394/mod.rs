use std::{cmp::Reverse, collections::BTreeMap};

pub struct Solution;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut frequency: BTreeMap<Reverse<i32>, i32> = BTreeMap::new();
        for i in arr.into_iter() {
            *frequency.entry(Reverse(i)).or_default() += 1;
        }
        for (Reverse(i), count) in frequency {
            if i == count {
                return i;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,2,3,4] => 2; "example 1")]
    #[test_case(vec![1,2,2,3,3,3] => 3; "example 2")]
    #[test_case(vec![2,2,2,2,3,3] => -1; "example 3")]
    fn test_solution(arr: Vec<i32>) -> i32 {
        Solution::find_lucky(arr)
    }
}
