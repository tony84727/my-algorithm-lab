pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![Vec::new(); triangle.len()];
        for (level, elements) in triangle.into_iter().enumerate() {
            for (i, &element) in elements.iter().enumerate() {
                if level == 0 {
                    dp[level].push(element);
                    continue;
                }
                let min = match i {
                    _ if i == 0 => dp[level - 1][i],
                    _ if i == elements.len() - 1 => dp[level - 1][i - 1],
                    _ => dp[level - 1][i].min(dp[level - 1][i - 1]),
                };
                dp[level].push(min + element);
            }
        }
        *dp.last().unwrap().iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[2],[3,4],[6,5,7],[4,1,8,3]] => 11; "example 1")]
    #[test_case(vec![vec![-10]] => -10; "example 2")]
    fn test_solution(triangle: Vec<Vec<i32>>) -> i32 {
        Solution::minimum_total(triangle)
    }
}
