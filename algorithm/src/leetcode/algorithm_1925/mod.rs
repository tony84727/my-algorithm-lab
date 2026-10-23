pub struct Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut count = 0;
        for i in 1..=n {
            for j in 1..=n {
                for k in 1..=n {
                    if i * i + j * j == k * k {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5 => 2; "example 1")]
    #[test_case(10 => 4; "example 2")]
    fn test_solution(n: i32) -> i32 {
        Solution::count_triples(n)
    }
}
