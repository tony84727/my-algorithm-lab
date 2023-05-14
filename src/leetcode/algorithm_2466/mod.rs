pub struct Solution;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let low = low as usize;
        let high = high as usize;
        let zero = zero as usize;
        let one = one as usize;
        let prime = 1000000007;
        let mut string_ways_of_lengths = vec![0; high + 1];
        string_ways_of_lengths[0] = 1;
        let mut ways = 0;
        for i in 1..=high {
            if zero <= i {
                string_ways_of_lengths[i] += string_ways_of_lengths[i - zero];
            }
            if one <= i {
                string_ways_of_lengths[i] += string_ways_of_lengths[i - one];
            }
            string_ways_of_lengths[i] %= prime;
            if low <= i && high >= i {
                ways += string_ways_of_lengths[i];
                ways %= prime;
            }
        }
        ways
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3,3,1,1 => 8; "example 1")]
    #[test_case(2,3,1,2 => 5; "example 2")]
    #[test_case(5,5,2,4 => 0; "case 1")]
    #[test_case(200,200,10,1 => 764262396; "case 2")]
    fn test_solution(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        Solution::count_good_strings(low, high, zero, one)
    }
}
