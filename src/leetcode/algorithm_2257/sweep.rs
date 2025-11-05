pub struct Solution;

#[derive(Clone)]
enum State {
    Guard,
    Wall,
    Empty(bool),
}

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut grid = vec![vec![State::Empty(false); n]; m];
        for element in walls.iter() {
            let r = element[0] as usize;
            let c = element[1] as usize;
            grid[r][c] = State::Wall;
        }
        for g in guards.into_iter() {
            let r = g[0] as usize;
            let c = g[1] as usize;
            grid[r][c] = State::Guard;
        }
        for row in grid.iter_mut() {
            let mut visible = false;
            for cell in row.iter_mut() {
                match cell {
                    State::Guard => {
                        visible = true;
                    }
                    State::Wall => {
                        visible = false;
                    }
                    State::Empty(guarded) if visible => {
                        *guarded = true;
                    }
                    _ => (),
                };
            }
            let mut visible = false;
            for cell in row.iter_mut().rev() {
                match cell {
                    State::Guard => {
                        visible = true;
                    }
                    State::Wall => {
                        visible = false;
                    }
                    State::Empty(guarded) if visible => {
                        *guarded = true;
                    }
                    _ => (),
                };
            }
        }
        for column in 0..n {
            let mut visible = false;
            for row in grid.iter_mut() {
                match &mut row[column] {
                    State::Guard => {
                        visible = true;
                    }
                    State::Wall => {
                        visible = false;
                    }
                    State::Empty(guarded) if visible => {
                        *guarded = true;
                    }
                    _ => (),
                }
            }
            let mut visible = false;
            for row in (0..m).rev() {
                match &mut grid[row][column] {
                    State::Guard => {
                        visible = true;
                    }
                    State::Wall => {
                        visible = false;
                    }
                    State::Empty(guarded) if visible => {
                        *guarded = true;
                    }
                    _ => (),
                }
            }
        }
        grid.into_iter()
            .map(|row| {
                row.into_iter()
                    .filter(|x| matches!(x, State::Empty(false)))
                    .count()
            })
            .sum::<usize>() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(4,6 , vecvec![[0,0],[1,1],[2,3]], vecvec![[0,1],[2,2],[1,4]] => 7; "example 1")]
    #[test_case(3,3,vec![vec![1,1]],vecvec![[0,1],[1,0],[2,1],[1,2]] => 4; "example 2")]
    #[test_case(2, 7, vecvec![[1,5],[1,1],[1,6],[0,2]], vecvec![[0,6],[0,3],[0,5]] => 1; "case 1")]
    fn test_solution(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        Solution::count_unguarded(m, n, guards, walls)
    }
}
