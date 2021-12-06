pub struct Solution;

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut odd = 0;
        let mut even = 0;
        for p in position.into_iter() {
            if p % 2 == 0 {
                odd += 1;
            } else {
                even += 1;
            }
        }
        if even > odd {
            odd
        } else {
            even
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3] => 1; "example 1")]
    #[test_case(vec![2,2,2,3,3] => 2; "example 2")]
    #[test_case(vec![1,1000000000] => 1; "example 3")]
    fn test_solution(position: Vec<i32>) -> i32 {
        Solution::min_cost_to_move_chips(position)
    }
}
