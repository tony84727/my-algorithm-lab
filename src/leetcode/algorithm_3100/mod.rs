pub struct Solution;

impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut drunk = 0;
        let mut full_bottles = num_bottles;
        while full_bottles >= num_exchange {
            full_bottles += 1 - num_exchange;
            drunk += num_exchange;
            num_exchange += 1;
        }
        drunk + full_bottles
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(13, 6 => 15; "example 1")]
    #[test_case(10, 3 => 13; "example 2")]
    fn test_solution(num_bottles: i32, num_exchange: i32) -> i32 {
        Solution::max_bottles_drunk(num_bottles, num_exchange)
    }
}
