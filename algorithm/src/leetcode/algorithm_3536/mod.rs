pub struct Solution;

impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let mut digits = vec![0; 10];
        while n > 0 {
            digits[(n % 10) as usize] += 1;
            n /= 10;
        }
        digits
            .into_iter()
            .enumerate()
            .rev()
            .filter(|(_i, exist)| *exist > 0)
            .flat_map(|(i, x)| vec![i as i32; x as usize])
            .take(2)
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(31 => 3; "example 1")]
    #[test_case(22 => 4; "example 2")]
    #[test_case(124 => 8; "example 3")]
    fn examples(n: i32) -> i32 {
        Solution::max_product(n)
    }
}
