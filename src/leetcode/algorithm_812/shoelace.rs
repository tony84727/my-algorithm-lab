pub struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut max = 0.0_f64;
        for (i, a) in points.iter().enumerate() {
            for (j, b) in points.iter().enumerate().skip(i + 1) {
                for c in points.iter().skip(j + 1) {
                    let (x1, y1) = (a[0], a[1]);
                    let (x2, y2) = (b[0], b[1]);
                    let (x3, y3) = (c[0], c[1]);
                    max = max
                        .max(0.5 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs() as f64)
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
