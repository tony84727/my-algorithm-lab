pub struct Solution;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut seat_index = Vec::new();
        for (i, c) in corridor.bytes().enumerate() {
            if c == b'S' {
                seat_index.push(i);
            }
        }
        if seat_index.is_empty() || !seat_index.len().is_multiple_of(2) {
            return 0;
        }
        let mut count = 1;
        for i in (2..seat_index.len()).step_by(2) {
            count = (count * (seat_index[i] - seat_index[i - 1])) % 1000000007;
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("SSPPSPS" => 3; "example 1")]
    #[test_case("PPSPSP" => 1; "example 2")]
    #[test_case("S" => 0; "example 3")]
    #[test_case("P" => 0; "case 1")]
    fn test_solution(corridor: &str) -> i32 {
        Solution::number_of_ways(String::from(corridor))
    }
}
