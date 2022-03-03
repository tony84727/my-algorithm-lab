pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let size = (board.len(), board.first().map(|x| x.len()).unwrap());
        fn find(
            board: &[Vec<char>],
            size: (usize, usize),
            from: (usize, usize),
            target: &str,
            mut visited: Vec<Vec<bool>>,
        ) -> bool {
            if target.is_empty() {
                return true;
            }
            visited[from.0][from.1] = true;
            let first = target.chars().next().unwrap();
            // top
            if from.0 > 0 {
                let next = (from.0 - 1, from.1);
                let &is_visited = visited
                    .get(next.0)
                    .and_then(|row| row.get(next.1))
                    .unwrap_or(&false);
                if !is_visited
                    && board[next.0][next.1] == first
                    && find(board, size, next, &target[1..], visited.clone())
                {
                    return true;
                }
            }
            // bottom
            if from.0 < size.0 - 1 {
                let next = (from.0 + 1, from.1);
                let &is_visited = visited
                    .get(next.0)
                    .and_then(|row| row.get(next.1))
                    .unwrap_or(&false);
                if !is_visited
                    && board[next.0][next.1] == first
                    && find(board, size, next, &target[1..], visited.clone())
                {
                    return true;
                }
            }
            // left
            if from.1 > 0 {
                let next = (from.0, from.1 - 1);
                let &is_visited = visited
                    .get(next.0)
                    .and_then(|row| row.get(next.1))
                    .unwrap_or(&false);
                if !is_visited
                    && board[next.0][next.1] == first
                    && find(board, size, next, &target[1..], visited.clone())
                {
                    return true;
                }
            }
            // right
            if from.1 < size.1 - 1 {
                let next = (from.0, from.1 + 1);
                let &is_visited = visited
                    .get(next.0)
                    .and_then(|row| row.get(next.1))
                    .unwrap_or(&false);
                if !is_visited
                    && board[next.0][next.1] == first
                    && find(board, size, next, &target[1..], visited.clone())
                {
                    return true;
                }
            }
            false
        }
        for h in 0..size.0 {
            for w in 0..size.1 {
                let begin = word.chars().next().unwrap();
                if board[h][w] == begin
                    && find(
                        &board,
                        size,
                        (h, w),
                        &word[1..],
                        vec![vec![false; size.1]; size.0],
                    )
                {
                    return true;
                }
            }
        }
        false
    }
}
