pub mod recursive;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3 => 5; "example 1")]
    #[test_case(2 => 2; "case 1")]
    fn test_recursive_solution(n: i32) -> i32 {
        recursive::Solution::num_trees(n)
    }
}
