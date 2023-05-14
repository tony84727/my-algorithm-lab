pub struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let n = nums.len() / 2;
        let mut max = 0;

        let mut pairs = nums.clone();
        pairs.rotate_right(n);
        for _ in 0..n {
            let mut gcd_results = nums
                .iter()
                .enumerate()
                .take(n)
                .map(|(i, a)| Self::gcd(*a, pairs[i]))
                .collect::<Vec<i32>>();
            gcd_results.sort();
            max = max.max(
                gcd_results
                    .into_iter()
                    .enumerate()
                    .map(|(i, n)| (i + 1) as i32 * n)
                    .sum(),
            );
            pairs.rotate_right(1);
        }
        max
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while a != 0 && b != 0 {
            if a > b {
                a -= b;
            } else {
                b -= a;
            }
        }
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2] => 1; "example 1")]
    #[test_case(vec![3,4,6,8] => 11; "example 2")]
    #[test_case(vec![1,2,3,4,5,6] => 14; "example 3")]
    #[test_case(vec![9,2] => 1; "case 1")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::max_score(nums)
    }

    #[test_case(1, 3 => 1)]
    #[test_case(3, 4 => 1)]
    #[test_case(9, 12 => 3)]
    fn test_gcd(a: i32, b: i32) -> i32 {
        Solution::gcd(a, b)
    }
}
