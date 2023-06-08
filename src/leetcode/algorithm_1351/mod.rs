pub struct Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        grid.into_iter().flatten().filter(|&x| x < 0).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![vec![4,3,2,-1],vec![3,2,1,-1],vec![1,1,-1,-2],vec![-1,-1,-2,-3]] => 8; "example 1")]
    fn test_solution(grid: Vec<Vec<i32>>) -> i32 {
        Solution::count_negatives(grid)
    }
}
