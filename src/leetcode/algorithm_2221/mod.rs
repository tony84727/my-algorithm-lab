pub struct Solution;

impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let mut current = nums;
        while current.len() != 1 {
            let batch = std::mem::take(&mut current);
            let mut iter = batch.into_iter();
            let mut a;
            let mut b = iter.next().unwrap();
            for f in iter {
                a = b;
                b = f;
                current.push((a + b) % 10);
            }
        }
        *current.first().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1, 2, 3, 4, 5] => 8; "example 1")]
    #[test_case(vec![5] => 5; "example 2")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::triangular_sum(nums)
    }
}
