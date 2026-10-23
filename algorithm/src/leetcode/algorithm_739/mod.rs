pub mod brute_force;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![73,74,75,71,69,72,76,73] => vec![1,1,4,2,1,1,0,0]; "example 1")]
    fn test_solution(temperatures: Vec<i32>) -> Vec<i32> {
        brute_force::Solution::daily_temperatures(temperatures)
    }
}
