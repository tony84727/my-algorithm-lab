pub struct Solution;

impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return vec![];
        }
        let mut direction: (i32, i32) = (1, 0);
        let mut current_position: (i32, i32) = (0, 0);
        let width = matrix.first().unwrap().len() as i32;
        let height = matrix.len() as i32;
        let length = matrix.len() * matrix.first().unwrap().len();
        let mut answer = Vec::with_capacity(length);
        fn turn_right(direction: (i32, i32)) -> (i32, i32) {
            let (x, y) = direction;
            (y, -x)
        }
        for _ in 0..length {
            let (x, y) = current_position;
            let (dx, dy) = direction;
            let mut current = 0;
            std::mem::swap(&mut current, &mut matrix[y as usize][x as usize]);
            answer.push(current);
            if (x == 0 && dx == -1)
                || (x == width - 1 && dx == 1)
                || (y == 0 && dy == 1)
                || (y == height - 1 && dy == -1)
                || matrix[(y - dy) as usize][(x + dx) as usize] == 0
            {
                direction = turn_right(direction);
            }
            let (dx, dy) = direction;
            current_position = (x + dx, y - dy);
        }
        answer
    }
}
