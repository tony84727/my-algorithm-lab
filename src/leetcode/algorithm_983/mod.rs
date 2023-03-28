pub struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut minimum = 0;
        let mut prices = vec![0, 0, 0];
        let mut until = vec![0, 0, 0];
        for d in days.into_iter() {
            prices[0] = minimum + costs[0];
            if d >= until[1] {
                until[1] = d + 7;
                prices[1] = minimum + costs[1];
            }
            if d >= until[2] {
                until[2] = d + 30;
                prices[2] = minimum + costs[2];
            }
            minimum = *prices.iter().min().unwrap();
        }
        minimum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,4,6,7,8,20],vec![2,7,15] => 11; "example 1")]
    #[test_case(vec![1,2,3,4,5,6,7,8,9,10,30,31], vec![2,7,15] => 17; "example 2")]
    #[test_case(vec![1,4,6], vec![1,4,20] => 3; "case 1")]
    #[test_case(vec![1,3,6], vec![1,4,20] => 3; "case 2")]
    #[test_case(vec![3,4,6,7,8,30], vec![2,7,15] => 9; "case 3")]
    #[test_case(vec![4,5,9,11,14,16,17,19,21,22,24], vec![1,4,18] => 10; "case 4")]
    fn test_solution(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        Solution::mincost_tickets(days, costs)
    }
}
