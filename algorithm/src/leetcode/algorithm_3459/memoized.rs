use std::collections::HashMap;

pub struct Solution;

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct Search {
    direction: usize,
    r: usize,
    c: usize,
    expected: i32,
    turned: bool,
}

impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut memoized = HashMap::new();
        for (r, row) in grid.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    for direction in 0..4 {
                        max = max.max(
                            1 + Self::find_lognest_v(
                                &mut memoized,
                                &grid,
                                Search {
                                    direction,
                                    r,
                                    c,
                                    expected: 2,
                                    turned: false,
                                },
                            ),
                        )
                    }
                }
            }
        }
        max
    }

    fn find_lognest_v(
        memoized: &mut HashMap<Search, i32>,
        grid: &[Vec<i32>],
        search: Search,
    ) -> i32 {
        if let Some(answer) = memoized.get(&search) {
            return *answer;
        }
        let Search {
            direction,
            r,
            c,
            expected,
            turned,
        } = search;
        let m = grid.len();
        let n = grid.first().unwrap().len();
        let directions = [(-1, 1), (1, 1), (1, -1), (-1, -1)];
        let offset = directions[direction];
        let turned_direction = (direction + 1) % 4;
        let next_row = r as i32 + offset.0;
        let next_column = c as i32 + offset.1;
        let next_expected = if expected == 2 { 0 } else { 2 };
        if !(0..(m as i32)).contains(&next_row)
            || !(0..(n as i32)).contains(&next_column)
            || grid[next_row as usize][next_column as usize] != expected
        {
            memoized.insert(search, 0);
            return 0;
        }
        let next_row = next_row as usize;
        let next_column = next_column as usize;
        if turned {
            let answer = 1 + Self::find_lognest_v(
                memoized,
                grid,
                Search {
                    direction,
                    r: next_row,
                    c: next_column,
                    expected: next_expected,
                    turned,
                },
            );
            memoized.insert(search, answer);
            return answer;
        }
        let answer = 1 + Self::find_lognest_v(
            memoized,
            grid,
            Search {
                direction: turned_direction,
                r: next_row,
                c: next_column,
                expected: next_expected,
                turned: true,
            },
        )
        .max(Self::find_lognest_v(
            memoized,
            grid,
            Search {
                direction,
                r: next_row,
                c: next_column,
                expected: next_expected,
                turned: false,
            },
        ));
        memoized.insert(search, answer);
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[2,2,1,2,2],[2,0,2,2,0],[2,0,1,1,0],[1,0,2,2,2],[2,0,0,2,2]] => 5; "example 1")]
    #[test_case(vecvec![[2,2,2,2,2],[2,0,2,2,0],[2,0,1,1,0],[1,0,2,2,2],[2,0,0,2,2]] => 4; "example 2")]
    #[test_case(vecvec![[1,2,2,2,2],[2,2,2,2,0],[2,0,0,0,0],[0,0,2,2,2],[2,0,0,2,0]] => 5; "example 3")]
    #[test_case(vec![vec![1]] => 1; "example 4")]
    #[test_case(vecvec![[2, 2, 0, 2, 0, 2, 0],[1, 2, 2, 1, 0, 2, 0]] => 2; "case 1")]
    fn test_solution(grid: Vec<Vec<i32>>) -> i32 {
        Solution::len_of_v_diagonal(grid)
    }
}
