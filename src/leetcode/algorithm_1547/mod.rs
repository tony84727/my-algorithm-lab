use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        cuts.sort();
        cuts.insert(0, 0);
        cuts.push(n);
        fn cost(
            cuts: &[i32],
            cache: &mut HashMap<(usize, usize), i32>,
            left: usize,
            right: usize,
        ) -> i32 {
            if right - left == 1 {
                return 0;
            }
            if let Some(answer) = cache.get(&(left, right)) {
                return *answer;
            }
            let answer = (left + 1..right)
                .map(|mid| {
                    cost(cuts, cache, left, mid) + cost(cuts, cache, mid, right) + cuts[right]
                        - cuts[left]
                })
                .min()
                .unwrap_or_default();
            cache.insert((left, right), answer);
            answer
        }
        let mut cache = HashMap::new();
        cost(&cuts, &mut cache, 0, cuts.len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(7, vec![3,5,1,4] => 16; "example 1")]
    fn test_solution(n: i32, cuts: Vec<i32>) -> i32 {
        Solution::min_cost(n, cuts)
    }
}
