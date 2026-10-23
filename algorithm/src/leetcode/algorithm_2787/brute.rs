pub struct Solution;

impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        let x = x as u32;
        let n = n as usize;
        Self::solve(n, x, n)
    }

    fn solve(n: usize, x: u32, max_int: usize) -> i32 {
        if n == 0 {
            return 1;
        }
        if max_int == 0 {
            return 0;
        }
        let mut count = 0;
        for p in 1..=max_int {
            let power = p.pow(x);
            if power > n {
                continue;
            }
            count += Self::solve(n - power, x, p - 1);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(10, 2 => 1; "example 1")]
    #[test_case(4, 1=> 2; "example 2")]
    fn test_solution(n: i32, x: i32) -> i32 {
        Solution::number_of_ways(n, x)
    }
}
