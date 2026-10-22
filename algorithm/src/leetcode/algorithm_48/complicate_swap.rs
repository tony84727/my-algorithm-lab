pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut [Vec<i32>]) {
        let len = matrix.len();
        let mid = len / 2;
        for layer in 0..mid {
            let layer_size = len - layer * 2;
            for cell in 0..layer_size - 1 {
                let mut cell_i = layer;
                let mut cell_j = layer + cell;
                for _ in 0..3 {
                    let next_cell_i = len - cell_j - 1;
                    let next_cell_j = cell_i;
                    matrix[cell_i][cell_j] ^= matrix[next_cell_i][next_cell_j];
                    matrix[next_cell_i][next_cell_j] ^= matrix[cell_i][cell_j];
                    matrix[cell_i][cell_j] ^= matrix[next_cell_i][next_cell_j];
                    cell_i = next_cell_i;
                    cell_j = next_cell_j;
                }
            }
        }
    }
}
