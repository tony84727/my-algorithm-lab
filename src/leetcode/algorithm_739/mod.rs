pub mod brute_force;
pub mod sliding_window;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![73,74,75,71,69,72,76,73] => vec![1,1,4,2,1,1,0,0]; "example 1")]
    fn test_brute_force_solution(temperatures: Vec<i32>) -> Vec<i32> {
        brute_force::Solution::daily_temperatures(temperatures)
    }

    #[test_case(vec![73,74,75,71,69,72,76,73] => vec![1,1,4,2,1,1,0,0]; "example 1")]
    #[test_case(vec![75,71,69,72,76,73] => vec![4,2,1,1,0,0]; "simplified example 1")]
    #[test_case(vec![89,62,70,58,47,47,46,76,100,70] => vec![8,1,5,4,3,2,1,1,0,0]; "case 1")]
    fn test_sliding_window_solution(temperatures: Vec<i32>) -> Vec<i32> {
        sliding_window::Solution::daily_temperatures(temperatures)
    }
}
