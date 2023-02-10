pub mod bfs;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[1,0,1], [0,0,0], [1,0,1]] => 2; "example 1")]
    #[test_case(vecvec![
        [1,1,1,1,1],
        [1,1,1,1,1],
        [1,1,1,1,1],
        [1,1,1,1,1],
        [1,1,1,1,1]
    ] => -1; "case 1")]
    fn test_solution(grid: Vec<Vec<i32>>) -> i32 {
        bfs::Solution::max_distance(grid)
    }
}
