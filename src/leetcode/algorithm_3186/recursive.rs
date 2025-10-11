use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let mut banned = HashMap::new();
        Self::solve(&mut banned, &power)
    }

    pub fn solve(banned: &mut HashMap<i32, usize>, power: &[i32]) -> i64 {
        if power.is_empty() {
            return 0;
        }

        let cast = if banned.contains_key(&power[0]) {
            0
        } else {
            for d in 1..=2 {
                for s in [-1, 1] {
                    *banned.entry(power[0] + d * s).or_default() += 1;
                }
            }
            let cast = power[0] as i64 + Self::solve(banned, &power[1..]);
            let mut to_remove = vec![];
            for d in 1..=2 {
                for s in [-1, 1] {
                    let to_ban = power[0] + d * s;
                    let c = banned.entry(to_ban).or_default();
                    if *c == 1 {
                        to_remove.push(to_ban);
                    } else {
                        *c -= 1;
                    }
                }
            }
            for r in to_remove.into_iter() {
                banned.remove(&r);
            }
            cast
        };
        let no_cast = Self::solve(banned, &power[1..]);
        cast.max(no_cast)
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
