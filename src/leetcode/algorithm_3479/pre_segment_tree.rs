use std::collections::{BTreeMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut basket_sizes: BTreeMap<i32, Vec<usize>> = Default::default();
        for (i, size) in baskets.into_iter().enumerate().rev() {
            basket_sizes.entry(size).or_default().push(i);
        }
        let sizes: Vec<i32> = basket_sizes.keys().cloned().collect();
        for window in sizes.windows(2).rev() {
            let larger = window[1];
            let smaller = window[0];
            let mut larger_positions = basket_sizes.get(&larger).unwrap().clone();
            let merged = basket_sizes.get_mut(&smaller).unwrap();
            let mut smaller_posiitons = std::mem::take(merged);
            larger_positions.reverse();
            smaller_posiitons.reverse();
            while !smaller_posiitons.is_empty() || !larger_positions.is_empty() {
                let mut with_larger = &mut smaller_posiitons;
                if with_larger.is_empty()
                    || with_larger.last().cloned().unwrap_or_default()
                        < larger_positions.last().cloned().unwrap_or_default()
                {
                    with_larger = &mut larger_positions
                }
                merged.push(with_larger.pop().unwrap());
            }
        }
        let mut used: HashSet<usize> = HashSet::new();
        let mut missed = 0;
        'outer: for f in fruits.into_iter() {
            let Some((_, positions)) = basket_sizes.range_mut(f..).next() else {
                missed += 1;
                continue;
            };
            while let Some(basket) = positions.pop() {
                if used.contains(&basket) {
                    continue;
                }
                used.insert(basket);
                continue 'outer;
            }
            missed += 1;
        }
        missed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![4,2,5], vec![3,5,4] => 1; "example 1")]
    #[test_case(vec![3,6,1], vec![6,4,7] => 0; "example 2")]
    #[test_case(vec![31], vec![6] => 1; "case 1")]
    #[test_case(vec![60,17,76], vec![99,48,82] => 0; "case 2")]
    fn test_solution(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        Solution::num_of_unplaced_fruits(fruits, baskets)
    }
}
