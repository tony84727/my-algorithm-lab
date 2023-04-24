use std::collections::BinaryHeap;
pub struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        for s in stones.into_iter() {
            heap.push(s);
        }
        while heap.len() > 1 {
            let x = heap.pop().unwrap();
            let y = heap.pop().unwrap();
            if x > y {
                heap.push(x - y);
            }
        }
        heap.pop().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,7,4,1,8,1] => 1; "example 1")]
    #[test_case(vec![2,2] => 0; "case 1")]
    fn test_solution(stones: Vec<i32>) -> i32 {
        Solution::last_stone_weight(stones)
    }
}
