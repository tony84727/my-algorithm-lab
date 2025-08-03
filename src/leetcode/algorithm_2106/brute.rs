use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let mut fruit_map = BTreeMap::new();
        for f in fruits.into_iter() {
            fruit_map.insert(f[0], f[1]);
        }
        Self::move_to(&mut fruit_map, start_pos, k)
    }

    fn move_to(fruits: &mut BTreeMap<i32, i32>, start_pos: i32, k: i32) -> i32 {
        if k < 0 || start_pos < 0 {
            return 0;
        }
        let current = fruits.remove(&start_pos);
        let next_left = fruits.range(..start_pos).next_back().map(|(k, _v)| *k);
        let next_right = fruits.range(start_pos..).next().map(|(k, _v)| *k);
        let move_left = next_left
            .map(|left| Self::move_to(fruits, left, k - (start_pos - left)))
            .unwrap_or_default();
        let move_right = next_right
            .map(|right| Self::move_to(fruits, right, k - (right - start_pos)))
            .unwrap_or_default();
        let next_max = move_left.max(move_right);
        match current {
            Some(current) => {
                let harvested = current + next_max;
                fruits.insert(start_pos, current);
                harvested
            }
            None => next_max,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[2,8],[6,3],[8,6]], 5, 4 => 9; "example 1")]
    #[test_case(vecvec![[0,9],[4,1],[5,7],[6,2],[7,4],[10,9]], 5, 4 => 14; "example 2")]
    fn test_solution(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        Solution::max_total_fruits(fruits, start_pos, k)
    }
}
