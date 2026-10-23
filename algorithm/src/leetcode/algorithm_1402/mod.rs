pub struct Solution;

impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort_unstable();
        satisfaction.reverse();
        let mut max = 0;
        let mut sum = 0;
        let mut last = 0;
        for n in satisfaction.into_iter() {
            sum += last + n;
            last += n;
            max = max.max(sum);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![-1,-8,0,5,-9] => 14;"example 1")]
    #[test_case(vec![4,3,2] => 20;"example 2")]
    #[test_case(vec![-1,-4,-5] => 0;"example 3")]
    fn test_solution(satisfaction: Vec<i32>) -> i32 {
        Solution::max_satisfaction(satisfaction)
    }
}
