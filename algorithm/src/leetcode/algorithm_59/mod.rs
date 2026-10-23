pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut direction: (i32, i32) = (1, 0);
        let mut current_position: (i32, i32) = (0, 0);
        let length = n as usize;
        let mut answer = vec![vec![0; length]; length];
        fn turn_right(direction: (i32, i32)) -> (i32, i32) {
            let (x, y) = direction;
            (y, -x)
        }
        for i in 0..(length * length) {
            let (x, y) = current_position;
            let (dx, dy) = direction;
            answer[y as usize][x as usize] = (i + 1) as i32;
            if (x == 0 && dx == -1)
                || (x == n - 1 && dx == 1)
                || (y == 0 && dy == 1)
                || (y == n - 1 && dy == -1)
                || answer[(y - dy) as usize][(x + dx) as usize] != 0
            {
                direction = turn_right(direction);
            }
            let (dx, dy) = direction;
            current_position = (x + dx, y - dy);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3 => vec![vec![1,2,3],vec![8,9,4],vec![7,6,5]]; "example 1")]
    fn test_solution(n: i32) -> Vec<Vec<i32>> {
        Solution::generate_matrix(n)
    }
}
