use std::{cmp::Ordering, collections::HashMap};

pub struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        (0..=n - k)
            .map(|start| {
                let mut frequency: HashMap<i32, usize> = HashMap::new();
                for num in nums.iter().skip(start).take(k) {
                    *frequency.entry(*num).or_default() += 1;
                }
                let mut entries: Vec<(i32, usize)> = frequency.into_iter().collect();
                entries.sort_by(|a, b| {
                    let count = b.1.cmp(&a.1);
                    if count != Ordering::Equal {
                        return count;
                    }
                    b.0.cmp(&a.0)
                });
                entries
                    .into_iter()
                    .take(x as usize)
                    .map(|x| x.0 * x.1 as i32)
                    .sum()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,1,2,2,3,4,2,3],6,2 => vec![6,10,12]; "example 1")]
    fn test_solution(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        Solution::find_x_sum(nums, k, x)
    }
}
