use std::{collections::HashSet, ops::Sub};

pub struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut a = HashSet::new();
        let mut b = HashSet::new();
        for n in nums1.into_iter() {
            a.insert(n);
        }
        for n in nums2.into_iter() {
            b.insert(n);
        }
        vec![
            a.sub(&b).iter().cloned().collect::<Vec<i32>>(),
            b.sub(&a).iter().cloned().collect::<Vec<i32>>(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    #[test_case(vec![1,2,3], vec![2,4,6] => vec![vec![3,1], vec![4,6]]; "example 1")]
    fn test_solution(a: Vec<i32>, b: Vec<i32>) -> Vec<Vec<i32>> {
        Solution::find_difference(a, b)
    }
}
