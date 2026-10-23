pub struct Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k > n {
            return 1.0;
        }
        Self::solve(n, k, max_pts)
    }

    pub fn solve(n: i32, k: i32, max_pts: i32) -> f64 {
        if k <= 0 {
            return if n < 0 { 0.0 } else { 1.0 };
        }
        if n <= 0 {
            return 0.0;
        }
        let mut chance = 0.0;
        for r in 1..=max_pts {
            chance += Self::solve(n - r, k - r, max_pts);
        }
        chance / max_pts as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(10, 1, 10 => 1.0; "example 1")]
    #[test_case(6, 1, 10 => 0.6; "example 2")]
    #[test_case(21, 17, 10 => 0.73278; "example 3")]
    fn test_solution(n: i32, k: i32, max_pts: i32) -> f64 {
        (Solution::new21_game(n, k, max_pts) * 100000.0).round() / 100000.0
    }
}
