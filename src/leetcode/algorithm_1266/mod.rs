pub struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points
            .iter()
            .enumerate()
            .take(points.len() - 1)
            .map(|(i, a)| Self::step(a, &points[i + 1]))
            .sum()
    }

    fn step(a: &[i32], b: &[i32]) -> i32 {
        let dx = (a[0] - b[0]).abs();
        let dy = (a[1] - b[1]).abs();
        let diagnose = dx.min(dy);
        dx + dy - diagnose
    }
}

#[cfg(test)]
mod tests {
    use crate::vecvec;

    use super::*;
    use test_case::test_case;

    #[test_case(vecvec![[1,1],[3,4],[-1,0]] => 7; "example 1")]
    fn test_solution(points: Vec<Vec<i32>>) -> i32 {
        Solution::min_time_to_visit_all_points(points)
    }
}
