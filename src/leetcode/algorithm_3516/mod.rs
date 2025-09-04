pub struct Solution;

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let xd = (z - x).abs();
        let yd = (z - y).abs();
        match xd.cmp(&yd) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Greater => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(2,7,4 => 1; "example 1")]
    #[test_case(2,5,6 => 2; "example 2")]
    #[test_case(1,5,3 => 0; "example 3")]
    fn test_solution(x: i32, y: i32, z: i32) -> i32 {
        Solution::find_closest(x, y, z)
    }
}
