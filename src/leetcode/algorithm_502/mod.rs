pub mod brute;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(2,0, vec![1,2,3], vec![0,1,1] => 4; "example 1")]
    #[test_case(3,0, vec![1,2,3], vec![0,1,2] => 6; "example 2")]
    #[test_case(1, 2, vec![1,2,3], vec![1,1,2] => 5; "case 1")]
    fn test_brute_solution(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        brute::Solution::find_maximized_capital(k, w, profits, capital)
    }
}
