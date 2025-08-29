pub struct Solution;

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let mut count = 0;
        for i in 1..=n {
            for j in 1..=m {
                if (i + j) % 2 != 0 {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3,2 => 3; "example 1")]
    #[test_case(1,1 => 0; "example 2")]
    fn test_solution(n: i32, m: i32) -> i64 {
        Solution::flower_game(n, m)
    }
}
