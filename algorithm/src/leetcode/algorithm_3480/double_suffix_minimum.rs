pub struct Solution;

impl Solution {
    pub fn max_subarrays(n: i32, mut conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        for p in conflicting_pairs.iter_mut() {
            p.sort_unstable();
        }
        let mut pair_suffix_minimum = vec![n + 1; n as usize + 1];
        let mut pair_suffix_second_minimum = vec![n + 1; n as usize + 1];
        for p in conflicting_pairs.into_iter() {
            let a = p[0] as usize;
            if pair_suffix_minimum[a] > p[1] {
                pair_suffix_second_minimum[p[0] as usize] = pair_suffix_minimum[p[0] as usize];
                pair_suffix_minimum[a] = p[1];
            } else if pair_suffix_second_minimum[a] > p[1] {
                pair_suffix_second_minimum[a] = p[1];
            }
        }
        let mut without_deletion = 0;
        let mut bonus = vec![0; n as usize + 1];
        let mut right = n + 1;
        let mut right_index = n as usize;
        for i in (1..=(n as usize)).rev() {
            if pair_suffix_minimum[right_index] > pair_suffix_minimum[i] {
                right = right
                    .min(pair_suffix_minimum[right_index])
                    .min(pair_suffix_second_minimum[i]);
                right_index = i;
            } else {
                right = right.min(pair_suffix_minimum[i]);
            }
            without_deletion += (pair_suffix_minimum[right_index] - i as i32) as i64;
            bonus[right_index] += (right - pair_suffix_minimum[right_index]) as i64;
        }
        without_deletion + bonus.into_iter().max().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::vecvec;

    use super::*;
    use test_case::test_case;

    #[test_case(4, vecvec![[2,3], [1,4]] => 9; "example 1")]
    #[test_case(5, vecvec![[1,2], [2,5], [3,5]] => 12; "example 2")]
    fn test_solution(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        Solution::max_subarrays(n, conflicting_pairs)
    }
}
