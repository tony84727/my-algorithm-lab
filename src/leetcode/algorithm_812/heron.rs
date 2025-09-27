pub struct Solution;

impl Solution {
    fn length(a: &[i32], b: &[i32]) -> f64 {
        (((a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2)) as f64).sqrt()
    }
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut max = 0.0_f64;
        for (i, a) in points.iter().enumerate() {
            for (j, b) in points.iter().enumerate().skip(i + 1) {
                for c in points.iter().skip(j + 1) {
                    let xl = Self::length(a, b);
                    let yl = Self::length(a, c);
                    let zl = Self::length(b, c);
                    let s = (xl + yl + zl) / 2.0;
                    let area = (s * (s - xl) * (s - yl) * (s - zl)).sqrt();
                    max = max.max(area);
                }
            }
        }
        (max * 100000.0).round() / 100000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[0,0],[0,1],[1,0],[0,2],[2,0]] => 2.0; "example 1")]
    #[test_case(vecvec![[1,0],[0,0],[0,1]] => 0.5; "example 2")]
    fn test_solution(points: Vec<Vec<i32>>) -> f64 {
        Solution::largest_triangle_area(points)
    }
}
