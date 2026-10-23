pub struct Solution;

impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let mut available = vec![0; skill.len()];
        let n = skill.len();
        for p in mana.iter() {
            let p = *p as i64;
            let mut now = available[0];
            for (i, &s) in skill.iter().enumerate().take(n - 1) {
                now = available[i + 1].max(now + (s as i64) * p)
            }
            *available.last_mut().unwrap() = now + (*skill.last().unwrap() as i64) * p;
            for i in (0..n - 1).rev() {
                available[i] = available[i + 1] - (skill[i + 1] as i64) * p;
            }
        }
        available.into_iter().last().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,5,2,4], vec![5,1,4,2] => 110; "example 1")]
    #[test_case(vec![1,1,1], vec![1,1,1] => 5; "example 2")]
    #[test_case(vec![1,2,3,4], vec![1,2] => 21; "example 3")]
    fn test_solution(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        Solution::min_time(skill, mana)
    }
}
