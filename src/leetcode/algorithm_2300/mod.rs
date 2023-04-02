pub mod brute;
pub mod sort;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![5,1,3], vec![1,2,3,4,5],7 => vec![4,0,3]; "example 1")]
    fn test_brute_solution(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        brute::Solution::successful_pairs(spells, potions, success)
    }
    #[test_case(vec![5,1,3], vec![1,2,3,4,5],7 => vec![4,0,3]; "example 1")]
    fn test_sort_solution(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        sort::Solution::successful_pairs(spells, potions, success)
    }
}
