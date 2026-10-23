pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
        let mut row_is_zero = vec![false; matrix.len()];
        let width = matrix.first().map(|x| x.len()).unwrap_or(0);
        let mut column_is_zero = vec![false; width];
        for (y, row_is_zero) in row_is_zero.iter_mut().enumerate() {
            for (x, column_is_zero) in column_is_zero.iter_mut().enumerate() {
                if matrix[y][x] == 0 {
                    *row_is_zero = true;
                    *column_is_zero = true;
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
                for row in matrix.iter_mut() {
                    row[column] = 0;
                }
            }
        }
    }
}
