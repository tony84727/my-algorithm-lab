use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max_heap = BinaryHeap::new();
        for mut n in nums.into_iter() {
            if n % 2 != 0 {
                n *= 2;
            }
            if n < min {
                min = n;
            }
            max_heap.push(n);
        }
        let mut min_deviation: Option<i32> = None;
        while let Some(current_max) = max_heap.pop() {
            let deviation = current_max - min;

            match min_deviation {
                Some(m) if m <= deviation => (),
                _ => {
                    min_deviation = Some(deviation);
                }
            }
            if current_max % 2 == 0 {
                let next = current_max / 2;
                if min > next {
                    min = next;
                }
                max_heap.push(next);
            } else {
                break;
            }
        }
        min_deviation.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,4] => 1; "example 1")]
    #[test_case(vec![4,1,5,20,3] => 3; "example 2")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::minimum_deviation(nums)
    }
}
