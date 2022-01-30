pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut [Vec<i32>]) {
        fn swap(matrix: &mut [Vec<i32>], ai: usize, aj: usize, bi: usize, bj: usize) {
            matrix[ai][aj] ^= matrix[bi][bj];
            matrix[bi][bj] ^= matrix[ai][aj];
            matrix[ai][aj] ^= matrix[bi][bj];
        }
        for row in 0..matrix.len() {
            for column in row + 1..matrix.len() {
                swap(matrix, row, column, column, row);
            }
        }
        for column in 0..matrix.len() / 2 {
            for row in 0..matrix.len() {
                swap(matrix, row, column, row, matrix.len() - 1 - column);
            }
        }
    }
}
