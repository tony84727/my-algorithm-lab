pub struct Solution;

impl Solution {
    // cartesian coordinate (x, y) -> (y, -x)
    // cartesian coordinate -> vector index
    // i =  c - y => y = c - i
    // j =  x + c => x = j - c
    // (-1,1) -> (1, 1)

    // (i, j) -> (c - y, x + c) -> (c + x, y + c) -> (c + (j - c), c - i + c) -> (j, 2*c - i)

    pub fn rotate(matrix: &mut [Vec<i32>]) {
        let len = matrix.len();
        let mid = len / 2;
        for layer in 0..mid {
            let layer_size = len - layer * 2;
            for cell in 0..layer_size - 1 {
                let mut cell_i = layer;
                let mut cell_j = layer + cell;
                let mut moving = matrix[cell_i][cell_j];
                for _ in 0..4 {
                    let next_cell_i = cell_j;
                    let next_cell_j = len - cell_i - 1;
                    std::mem::swap(&mut matrix[next_cell_i][next_cell_j], &mut moving);
                    cell_i = next_cell_i;
                    cell_j = next_cell_j;
                }
            }
        }
    }
}
