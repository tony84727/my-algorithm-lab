use std::collections::BinaryHeap;
pub struct Solution;

#[derive(Eq)]
struct Pair {
    value: i32,
    position: usize,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut min_heap: BinaryHeap<Pair> = BinaryHeap::new();
        for (position, value) in nums.into_iter().enumerate() {
            min_heap.push(Pair { value, position });
            if min_heap.len() > k {
                min_heap.pop();
            }
        }
        let mut pairs: Vec<Pair> = min_heap.into_iter().collect();
        pairs.sort_by_key(|x| x.position);
        pairs.into_iter().map(|x| x.value).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,1,3,3], 2 => vec![3,3]; "example 1")]
    #[test_case(vec![-1,-2,3,4], 3 => vec![-1, 3, 4]; "example 2")]
    fn test_solutions(nums: Vec<i32>, k: i32) -> Vec<i32> {
        Solution::max_subsequence(nums, k)
    }
}
