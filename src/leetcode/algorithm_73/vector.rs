pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut row_is_zero = vec![false; matrix.len()];
        let width = matrix.first().map(|x| x.len()).unwrap_or(0);
        let mut column_is_zero = vec![false; width];
        for y in 0..matrix.len() {
            for x in 0..width {
                if matrix[y][x] == 0 {
                    row_is_zero[y] = true;
                    column_is_zero[x] = true;
                }
            }
        }
        for (row, zero) in row_is_zero.into_iter().enumerate() {
            if zero {
                matrix[row].fill(0);
            }
        }
        for (column, zero) in column_is_zero.into_iter().enumerate() {
            if zero {
                for y in 0..matrix.len() {
                    matrix[y][column] = 0;
                }
            }
        }
    }
}
