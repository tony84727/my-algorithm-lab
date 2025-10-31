pub struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let find_largest_rectangle = |row: usize, column: usize| -> i32 {
            let mut max_area = 1;
            let mut max_width = matrix[0].len();
            for (r, current_row) in matrix.iter().enumerate().skip(row) {
                let height = r - row + 1;
                for (c, &cell) in current_row.iter().enumerate().skip(column) {
                    if c >= max_width {
                        break;
                    }
                    if cell != '1' {
                        max_width = c;
                        break;
                    }
                    let area = ((c - column + 1) * height) as i32;
                    if area > max_area {
                        max_area = area;
                    }
                }
            }
            max_area
        };
        let mut maximal = 0;
        for (r, row) in matrix.iter().enumerate() {
            for (column, &cell) in row.iter().enumerate() {
                if cell == '1' {
                    let area = find_largest_rectangle(r, column);
                    if area > maximal {
                        maximal = area;
                    }
                }
            }
        }
        maximal
    }
}
