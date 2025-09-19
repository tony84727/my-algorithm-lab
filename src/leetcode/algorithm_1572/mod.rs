pub struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let length = mat.len();
        let mut sum = 0;
        for n in 0..length / 2 {
            sum += mat[n][n];
            sum += mat[n][length - n - 1];
            sum += mat[length - n - 1][n];
            sum += mat[length - n - 1][length - n - 1];
        }
        if !length.is_multiple_of(2) {
            let i = length / 2;
            sum += mat[i][i];
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]] => 25; "example 1")]
    #[test_case(vec![
                vec![1,1,1,1],
                vec![1,1,1,1],
                vec![1,1,1,1],
                vec![1,1,1,1],
    ] => 8; "example 2")]
    #[test_case(vec![vec![5]] => 5; "example 3")]
    fn test_solution(mat: Vec<Vec<i32>>) -> i32 {
        Solution::diagonal_sum(mat)
    }
}
