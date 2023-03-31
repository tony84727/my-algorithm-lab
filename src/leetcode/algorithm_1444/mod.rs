pub struct Solution;

impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let row = pizza.len();
        let column = pizza.first().map(|x| x.len()).unwrap();
        let apples = Self::apples(pizza);
        let mut dp = vec![vec![vec![0; column]; row]; k as usize];
        for i in 0..row {
            for j in 0..column {
                dp[0][i][j] = if apples[i][j] > 0 { 1 } else { 0 };
            }
        }
        for remain in 1..(k as usize) {
            for i in 0..row {
                for j in 0..column {
                    let mut ways = 0_i64;
                    for next_row in i..row {
                        if apples[i][j] - apples[next_row][j] > 0 {
                            ways += dp[remain - 1][next_row][j] as i64;
                        }
                    }
                    for next_column in j..column {
                        if apples[i][j] - apples[i][next_column] > 0 {
                            ways += dp[remain - 1][i][next_column] as i64;
                        }
                    }
                    dp[remain][i][j] = (ways % 1000000007) as i32;
                }
            }
        }
        dp[(k - 1) as usize][0][0]
    }

    fn apples(pizza: Vec<String>) -> Vec<Vec<i32>> {
        let row = pizza.len();
        let column = pizza.first().map(|x| x.len()).unwrap();
        let mut counts = vec![vec![0; column + 1]; row + 1];
        for (i, row) in pizza.iter().enumerate().rev() {
            for (j, cell) in row.char_indices().rev() {
                counts[i][j] =
                    if cell == 'A' { 1 } else { 0 } + counts[i + 1][j] + counts[i][j + 1]
                        - counts[i + 1][j + 1];
            }
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec!["A..", "AAA", "..."], 3 => 3; "example 1")]
    fn test_solution(pizza: Vec<&'static str>, k: i32) -> i32 {
        let mut owned = vec![];
        for row in pizza.into_iter() {
            owned.push(row.to_owned());
        }
        Solution::ways(owned, k)
    }
}
