pub struct Solution;

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let (n_odd, n_even) = if n % 2 == 0 {
            (n / 2, n / 2)
        } else {
            ((n / 2) + 1, n / 2)
        };
        let (m_odd, m_even) = if m % 2 == 0 {
            (m / 2, m / 2)
        } else {
            ((m / 2) + 1, m / 2)
        };
        n_odd as i64 * m_even as i64 + n_even as i64 * m_odd as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3,2 => 3; "example 1")]
    #[test_case(1,1 => 0; "example 2")]
    #[test_case(58280, 69389 => 2021995460; "case 1")]
    fn test_solution(n: i32, m: i32) -> i64 {
        Solution::flower_game(n, m)
    }
}
