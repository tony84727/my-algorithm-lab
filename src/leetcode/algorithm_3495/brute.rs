pub struct Solution;

impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        let mut count = 0;
        for q in queries.into_iter() {
            let mut query_count = 0;
            for n in q[0]..=q[1] {
                query_count += ((n as f32).log(4.0) + 1.0) as i64;
            }
            count += ((query_count as f32) / 2.0).ceil() as i64
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[1,2],[2,4]] => 3; "example 1")]
    #[test_case(vec![vec![2,6]] => 4; "example 2")]
    #[test_case(vec![vec![1,8]] => 7; "case 1")]
    fn test_solution(queries: Vec<Vec<i32>>) -> i64 {
        Solution::min_operations(queries)
    }
}
