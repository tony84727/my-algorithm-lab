use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let mut frequency: BTreeMap<i32, usize> = BTreeMap::new();
        for p in power.into_iter() {
            *frequency.entry(p).or_default() += 1;
        }
        let mut index = vec![];
        for &k in frequency.keys() {
            index.push(k);
        }
        let m = index.len();
        let mut dp = vec![0; m];
        for i in 0..m {
            let current_power = index[i];
            let last_index = match index.binary_search(&(current_power - 2)) {
                Ok(i) if i > 0 => Some(i - 1),
                Err(i) if i > 0 => Some(i - 1),
                _ => None,
            };
            let current = current_power as i64 * *frequency.get(&current_power).unwrap() as i64;
            dp[i] = current
                + if let Some(last_index) = last_index {
                    dp[last_index]
                } else {
                    0
                };
        }
        dp.into_iter().max().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,1,3,4] => 6; "example 1")]
    #[test_case(vec![7,1,6,6] => 13; "example 2")]
    #[test_case(vec![5,9,2,10,2,7,10,9,3,8] => 31; "example 3")]
    fn test_solution(power: Vec<i32>) -> i64 {
        Solution::maximum_total_damage(power)
    }
}
