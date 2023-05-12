pub mod recursive;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![vec![3,2], vec![4,3], vec![4,4], vec![2,5]] => 5; "example 1")]
    fn test_recursive(questions: Vec<Vec<i32>>) -> i64 {
        recursive::Solution::most_points(questions)
    }
}
