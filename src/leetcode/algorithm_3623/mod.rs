use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let mut groups: HashMap<i64, i64> = HashMap::new();
        for p in points.into_iter() {
            let y = p[1] as i64;
            *groups.entry(y).or_default() += 1;
        }
        let modulo = 1_000_000_007;
        let mut count = 0;
        let mut last = 0;
        for group in groups.into_values() {
            if group < 2 {
                continue;
            }
            let combinations = group * (group - 1) / 2;
            count = (count + combinations * last) % modulo;
            last = (last + combinations) % modulo;
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[1,0],[2,0],[3,0],[2,2],[3,2]] => 3; "example 1")]
    #[test_case(vecvec![[0,0],[1,0],[0,1],[2,1]] => 1; "example 2")]
    #[test_case(vecvec![[-73,-72],[-1,-56],[-92,30],[-57,-89],[-19,-89],[-35,30],[64,-72]] => 3; "case 1")]
    fn test_solution(points: Vec<Vec<i32>>) -> i32 {
        Solution::count_trapezoids(points)
    }
}
