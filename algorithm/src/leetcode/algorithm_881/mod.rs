pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut left = 0;
        let mut right = people.len();
        let mut count = 0;
        while left < right {
            if people[left] + people[right - 1] > limit {
                count += 1;
                right -= 1;
            } else {
                right -= 1;
                left += 1;
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2], 3 => 1; "example 1")]
    #[test_case(vec![3,2,2,1], 3 => 3; "example 2")]
    #[test_case(vec![3,5,3,4], 5 => 4; "example 3")]
    fn test_solution(people: Vec<i32>, limit: i32) -> i32 {
        Solution::num_rescue_boats(people, limit)
    }
}
