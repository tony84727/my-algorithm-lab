pub struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut x = 0;
        for o in operations {
            if o.contains("+") {
                x += 1;
            } else {
                x -= 1;
            }
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec!["--X","X++","X++"] => 1; "example 1")]
    #[test_case(vec!["++X","++X","X++"] => 3; "example 2")]
    #[test_case(vec!["X++","++X","--X","X--"] => 0; "example 3")]
    fn test_solution(operations: Vec<&str>) -> i32 {
        Solution::final_value_after_operations(operations.into_iter().map(String::from).collect())
    }
}
