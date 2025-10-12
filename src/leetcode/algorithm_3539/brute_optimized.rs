pub struct Solution;

const MODULO: i64 = 1000000007;

impl Solution {
    pub fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let k = k as u32;
        let mut answer = 0;
        Self::run_sequence(&mut answer, &nums, n, m as usize, k, 0, 1);
        answer as i32
    }

    fn run_sequence(
        answer: &mut i64,
        nums: &[i32],
        n: usize,
        m: usize,
        k: u32,
        sum: i64,
        product: i64,
    ) {
        if m == 0 {
            if sum.count_ones() == k {
                *answer += product;
                *answer %= MODULO
            }
            return;
        }
        for head in 0..n {
            let number = 1 << head;
            let next_sum = sum + number;
            Self::run_sequence(
                answer,
                nums,
                n,
                m - 1,
                k,
                next_sum,
                (product * nums[head] as i64) % MODULO,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5,5, vec![1,10,100,10000,1000000] => 991600007; "example 1")]
    #[test_case(2,2, vec![5,4,3,2,1] => 170; "example 2")]
    #[test_case(4,2, vec![41] => 0; "case 1")]
    #[test_case(2,1, vec![63] => 3969; "case 2")]
    #[test_case(5,2, vec![24,2] => 11282336; "case 3")]
    fn test_solution(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        Solution::magical_sum(m, k, nums)
    }
}
