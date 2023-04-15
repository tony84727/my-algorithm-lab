pub mod divide_and_conquer;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![vec![1,100,3],vec![7,8,9]], 2 => 101; "example 1")]
    #[test_case(vec![vec![100],vec![100],vec![100],vec![100],vec![100],vec![100],vec![1,1,1,1,1,1,700]], 7 => 706; "example 2")]
    fn test_solution(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        divide_and_conquer::Solution::max_value_of_coins(piles, k)
    }
}
