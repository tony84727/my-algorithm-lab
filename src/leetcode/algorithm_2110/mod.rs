pub struct Solution;

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut count = 0;
        let mut length = 0;
        let Some(mut last) = prices.first().cloned() else {
            return 0;
        };
        for n in prices.into_iter() {
            if last != n + 1 {
                length = 0;
            }
            length += 1;
            count += length;
            last = n;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,2,1,4] => 7; "example 1")]
    #[test_case(vec![8,6,7,7] => 4; "example 2")]
    fn test_solution(prices: Vec<i32>) -> i64 {
        Solution::get_descent_periods(prices)
    }
}
