pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let size = (board.len(), board.first().map(|x| x.len()).unwrap());
        fn find(
            board: &[Vec<char>],
            size: (usize, usize),
            from: (usize, usize),
            target: &str,
        ) -> bool {
            if target.is_empty() {
                return false;
            }
            let first = target.chars().nth(0).unwrap();
            // top
            if from.0 > 0 && board[from.0 - 1][from.1] == first {
                if find(board, size, (from.0 - 1, from.1), &target[1..]) {
                    return true;
                }
            }
            // bottom
            if from.0 < size.0 - 1 && board[from.0 + 1][from.1] == first {
                if find(board, size, (from.0 + 1, from.1), &target[1..]) {
                    return true;
                }
            }
            // left
            if from.1 > 0 && board[from.0][from.1 - 1] == first {
                if find(board, size, (from.0, from.1 - 1), &target[1..]) {
                    return true;
                }
            }
            // right
            if from.1 < size.1 - 1 && board[from.0][from.1 + 1] == first {
                if find(board, size, (from.0, from.1 + 1), &target[1..]) {
                    return true;
                }
            }
            false
        }
        for w in 0..size.0 {
            for h in 0..size.1 {
                if find(&board, size, (w, h), &word) {
                    return true;
                }
            }
        }
        false
    }
}
