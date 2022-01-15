pub mod intuitive;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]] => vec![1,2,3,6,9,8,7,4,5]; "example 1")]
    fn test_intuitive_solution(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        intuitive::Solution::spiral_order(matrix)
    }
}
