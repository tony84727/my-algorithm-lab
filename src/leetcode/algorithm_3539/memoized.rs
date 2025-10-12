use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let k = k as u32;
        let mut memoized = vec![HashMap::new(); (m + 1) as usize];
        let sequences: HashSet<Vec<usize>> =
            HashSet::from_iter(Self::build_sequence(&mut memoized, n, m as usize, k, 0));
        let modulo: i64 = 1000000007;
        sequences.into_iter().fold(0, |a, c| {
            (a + c
                .into_iter()
                .fold(1, |acc, c| (acc * nums[c] as i64) % modulo))
                % modulo
        }) as i32
    }

    fn build_sequence(
        memoized: &mut [HashMap<i64, Vec<Vec<usize>>>],
        n: usize,
        m: usize,
        k: u32,
        sum: i64,
    ) -> Vec<Vec<usize>> {
        if let Some(cached) = &memoized[m].get(&sum) {
            return cached.to_vec();
        }
        if m == 0 {
            if sum.count_ones() == k {
                memoized[m].insert(sum, vec![Vec::new()]);
                return vec![Vec::new()];
            }
            memoized[m].insert(sum, Vec::new());
            return Vec::new();
        }
        let mut sequences = Vec::new();
        for head in 0..n {
            let number = 1 << head;
            let next_sum = sum + number;
            let subsequence = Self::build_sequence(memoized, n, m - 1, k, next_sum);
            for mut sub in subsequence.into_iter() {
                sub.insert(0, head);
                sequences.push(sub);
            }
        }
        memoized[m].insert(sum, sequences.clone());
        sequences
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5,5, vec![1,10,100,10000,1000000] => 991600007; "example 1")]
    #[test_case(2,2, vec![5,4,3,2,1] => 170; "example 2")]
    #[test_case(4,2, vec![41] => 0; "case 1")]
    #[test_case(2,1, vec![63] => 3969; "case 2")]
    #[test_case(5,2, vec![24,2] => 11282336; "case 3")]
    fn test_solution(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        Solution::magical_sum(m, k, nums)
    }
}
