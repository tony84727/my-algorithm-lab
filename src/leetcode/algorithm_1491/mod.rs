pub struct Solution;

impl Solution {
    pub fn average(mut salary: Vec<i32>) -> f64 {
        let length = salary.len();
        salary.sort_unstable();
        salary.into_iter().take(length - 1).skip(1).sum::<i32>() as f64 / (length - 2) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![4000,3000,1000,2000] => 2500.0; "example 1")]
    fn test_solution(salary: Vec<i32>) -> f64 {
        Solution::average(salary)
    }
}
