pub mod dfs_and_stack;
pub mod multi_dfs;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,3,4,2,3] => 3; "example 1")]
    #[test_case(vec![1,2,0,4,5,6,3,8,9,7] => 4; "case 1")]
    fn test_multi_dfs_solution(edges: Vec<i32>) -> i32 {
        multi_dfs::Solution::longest_cycle(edges)
    }

    #[test_case(vec![3,3,4,2,3] => 3; "example 1")]
    #[test_case(vec![1,2,0,4,5,6,3,8,9,7] => 4; "case 1")]
    fn test_dfs_with_stack_solution(edges: Vec<i32>) -> i32 {
        dfs_and_stack::Solution::longest_cycle(edges)
    }
}
