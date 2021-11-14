pub mod nxcbn;
pub mod brute_force;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(4,2 => vec![vec![2,4],vec![3,4], vec![2,3], vec![1,2], vec![1,3], vec![1,4]]; "example 1")]
    fn test_brute_force_solution(n: i32, length: i32) -> Vec<Vec<i32>> {
        brute_force::Solution::combine(n, lengh)
    }
}
