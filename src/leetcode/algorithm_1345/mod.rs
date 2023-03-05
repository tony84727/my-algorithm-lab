use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn reachable(
        connected: &HashMap<i32, Vec<usize>>,
        arr: &[i32],
        i: usize,
    ) -> Option<Vec<usize>> {
        let target = arr.len() - 1;
        let mut reachable = vec![];
        if i > 0 {
            reachable.push(i - 1);
        }
        if i < arr.len() - 1 {
            reachable.push(i + 1);
            if i + 1 == target {
                return None;
            }
        }
        for &next in connected.get(&arr[i]).unwrap().iter().filter(|x| **x != i) {
            if next == target {
                return None;
            }
            reachable.push(next);
        }
        Some(reachable)
    }
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        if arr.len() <= 1 {
            return 0;
        }
        let mut step = 1;
        let mut to_visit = vec![0];
        let mut connected = HashMap::<i32, Vec<usize>>::new();
        for (i, n) in arr.iter().enumerate() {
            connected.entry(*n).or_default().push(i);
        }
        'stepping: loop {
            let current = std::mem::replace(&mut to_visit, vec![]);
            for i in current.into_iter() {
                let next_step = Self::reachable(&connected, &arr, i);
                match next_step {
                    Some(mut next_step) => {
                        to_visit.append(&mut next_step);
                    }
                    None => break 'stepping,
                }
            }
            step += 1;
        }
        step
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![100,-23,-23,404,100,23,23,23,3,404] => 3; "example 1")]
    #[test_case(vec![7] => 0; "example 2")]
    #[test_case(vec![7,6,9,6,9,6,9,7] => 1; "example 3")]
    fn test_solution(arr: Vec<i32>) -> i32 {
        Solution::min_jumps(arr)
    }
}
