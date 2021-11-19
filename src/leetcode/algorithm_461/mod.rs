pub struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut diff = x ^ y;
        let mut distance = 0;
        for _ in 0..32 {
            if diff & 1 > 0 {
                distance += 1;
            }
            diff >>= 1;
        }
        distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1,4 => 2; "example 1")]
    #[test_case(3,1 => 1; "example 2")]
    fn test_solution(x: i32, y: i32) -> i32 {
        Solution::hamming_distance(x, y)
    }
}
