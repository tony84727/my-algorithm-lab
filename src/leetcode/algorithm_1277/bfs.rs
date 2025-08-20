pub struct Solution;

impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let max_row = matrix.len() - 1;
        let max_column = matrix.first().unwrap().len() - 1;
        let mut todo: Vec<(usize, usize)> = vec![];
        for (r, row) in matrix.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    todo.push((r, c));
                }
            }
        }
        let mut extension = 1;
        while !todo.is_empty() {
            let batch = std::mem::take(&mut todo);
            'processing: for (r, c) in batch.into_iter() {
                let last_column = c + extension;
                if last_column > max_column {
                    continue;
                }
                let last_row = r + extension;
                if last_row > max_row {
                    continue;
                }
                for row in matrix.iter().skip(r).take(extension + 1) {
                    if row[last_column] == 0 {
                        continue 'processing;
                    }
                }
                for column in c..c + extension {
                    if matrix[last_row][column] == 0 {
                        continue 'processing;
                    }
                }
                matrix[r][c] += 1;
                todo.push((r, c));
            }
            extension += 1;
        }
        matrix.iter().flatten().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::vecvec;

    use super::*;
    use test_case::test_case;

    #[test_case(vecvec![[0,1,1,1], [1,1,1,1], [0,1,1,1]] => 15; "example 1")]
    #[test_case(vecvec![[1,0,1], [1,1,0], [1,1,0]] => 7; "example 2")]
    fn test_solution(matrix: Vec<Vec<i32>>) -> i32 {
        Solution::count_squares(matrix)
    }
}
