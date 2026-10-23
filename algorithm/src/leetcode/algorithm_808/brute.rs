pub struct Solution;

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        let results = Self::outcome(n, n);
        let total: f64 = results.iter().sum();
        results[0] / total + results[2] / total / 2.0
    }
    fn outcome(a: i32, b: i32) -> [f64; 3] {
        if a <= 0 && b <= 0 {
            return [0.0, 0.0, 1.0];
        }
        if a <= 0 {
            return [1.0, 0.0, 0.0];
        }
        if b <= 0 {
            return [0.0, 1.0, 0.0];
        }
        [
            Self::outcome(a - 100, b),
            Self::outcome(a - 75, b - 25),
            Self::outcome(a - 50, b - 50),
            Self::outcome(a - 25, b - 75),
        ]
        .iter()
        .map(|x| x.map(|y| y * 0.25))
        .fold([0.0, 0.0, 0.0], |acc, c| {
            [acc[0] + c[0], acc[1] + c[1], acc[2] + c[2]]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(50 => 0.625; "example 1")]
    #[test_case(100 => 0.71875; "example 2")]
    fn test_solution(n: i32) -> f64 {
        Solution::soup_servings(n)
    }
}
