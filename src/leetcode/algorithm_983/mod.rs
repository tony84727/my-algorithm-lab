pub struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut minimums = vec![*costs.iter().min().unwrap()];
        let mut lookback: [i32; 2] = [-1, -1];
        for (i, d) in days.iter().enumerate().skip(1) {
            while ((lookback[0] + 1) as usize) < i && d - (if lookback[0] == -1 {0} else {days[lookback[0] as usize]}) > 7 {
                lookback[0] += 1;
            }
            while ((lookback[1] + 1) as usize) < i && d - (if lookback[1] == -1 {0} else {days[lookback[1] as usize]}) > 30 {
                lookback[1] += 1;
            }
            let mut min = minimums[i - 1] + costs[0];
            if lookback[0] == -1 {
                min = min.min(costs[1]);
            }else if d - days[lookback[0] as usize] <= 7 {
                min = min.min(minimums[lookback[0] as usize] + costs[1]);
            }
            if lookback[1] == -1 {
                min = min.min(costs[2]);
            } else if d - days[lookback[1] as usize] <= 30 {
                min = min.min(minimums[lookback[1] as usize] + costs[2]);
            }
            minimums.push(min);
            println!("{minimums:?}");
        }
        *minimums.last().unwrap()
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
