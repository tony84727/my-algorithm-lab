pub struct Solution;

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut count = num_bottles;
        let mut current = num_bottles;
        while current / num_exchange >= 1 {
            count += current / num_exchange;
            current += current / num_exchange - (current / num_exchange) * num_exchange;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(9,3 => 13; "example 1")]
    #[test_case(15, 4 => 19; "example 2")]
    fn test_solution(num_bottles: i32, num_exchange: i32) -> i32 {
        Solution::num_water_bottles(num_bottles, num_exchange)
    }
}
