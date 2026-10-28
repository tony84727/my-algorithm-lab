pub struct Solution;

#[derive(Copy, Clone, PartialEq)]
enum State {
    Odd,
    Even,
    Both,
}

const INIT_STATE: [State; 10] = [
    State::Even,
    State::Even,
    State::Even,
    State::Even,
    State::Even,
    State::Even,
    State::Even,
    State::Even,
    State::Even,
    State::Even,
];

fn to_state(mut x: i32) -> [State; 10] {
    let mut state = INIT_STATE;
    while x != 0 {
        let index = x.trailing_zeros();
        state[index as usize] = State::Odd;
        x &= x - 1;
    }
    state
}

fn combine(a: [State; 10], b: [State; 10], mut current: [State; 10]) -> [State; 10] {
    for i in 0..10 {
        let a = match a[i] {
            State::Odd if current[i] == State::Odd => State::Even,
            State::Even if current[i] == State::Odd => State::Odd,
            x => x,
        };
        let b = match b[i] {
            State::Odd if current[i] == State::Odd => State::Even,
            State::Even if current[i] == State::Odd => State::Odd,
            x => x,
        };
        if a == State::Both
            || b == State::Both
            || (a == State::Odd && b == State::Even)
            || (a == State::Even && b == State::Odd)
        {
            current[i] = State::Both;
        }
    }
    current
}

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let r = grid.len();
        let c = grid.first().map(|row| row.len()).unwrap();
        let mut candidates: Vec<Vec<[State; 10]>> = vec![vec![INIT_STATE; c]; r];
        candidates[0][0] = to_state(grid[0][0]);
        for i in 0..r {
            for j in 0..c {
                let current = to_state(grid[i][j]);
                if i == 0 && j == 0 {
                    candidates[0][0] = to_state(grid[0][0]);
                    continue;
                }
                let mut up = INIT_STATE;
                let mut left = INIT_STATE;
                if i > 0 {
                    up = candidates[i - 1][j];
                }
                if j > 0 {
                    left = candidates[i][j - 1];
                }
                candidates[i][j] = combine(up, left, current);
            }
        }
        let goal = to_state(grid[r - 1][c - 1]);
        let mut sum = 0;
        for (i, &s) in candidates[r - 1][c - 1].iter().enumerate() {
            if s == State::Both {
                continue;
            }
            if (s == State::Odd && goal[i] == State::Even)
                || (s == State::Even && goal[i] == State::Odd)
            {
                sum |= 1 << i;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[1, 2], [3, 4]] => 6; "example 1")]
    #[test_case(vecvec![[6, 7], [5, 8]] => 9; "example 2")]
    #[test_case(vec![vec![2, 7, 5]] => 0; "example 3")]
    fn test_min_cost(grid: Vec<Vec<i32>>) -> i32 {
        Solution::min_cost(grid)
    }
}
