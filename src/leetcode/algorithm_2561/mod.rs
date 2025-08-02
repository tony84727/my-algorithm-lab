use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut min = i32::MAX;
        let mut frequency: HashMap<i32, i32> = HashMap::new();
        for &n in basket1.iter() {
            *frequency.entry(n).or_default() += 1;
            min = min.min(n);
        }
        for &n in basket2.iter() {
            *frequency.entry(n).or_default() -= 1;
            min = min.min(n);
        }
        let mut imbalance = Vec::new();
        for (k, v) in frequency.into_iter() {
            if v % 2 != 0 {
                return -1;
            }
            for _ in 0..v.abs() / 2 {
                imbalance.push(k);
            }
        }
        imbalance.sort_unstable();
        Self::minimum_cost_of_rebalance(min, &imbalance)
    }
    fn minimum_cost_of_rebalance(min: i32, imbalance: &[i32]) -> i64 {
        let mut cost = 0_i64;
        for &n in &imbalance[0..imbalance.len() / 2] {
            cost += (min * 2).min(n) as i64;
        }
        cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![4,2,2,2], vec![1,4,1,2] => 1; "example 1")]
    #[test_case(vec![2,3,4,1], vec![3,2,5,1] => -1; "example 2")]
    #[test_case(vec![84,80,43,8,80,88,43,14,100,88], vec![32,32,42,68,68,100,42,84,14,8] => 48; "case 1")]
    fn test_solution(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        Solution::min_cost(basket1, basket2)
    }
}
