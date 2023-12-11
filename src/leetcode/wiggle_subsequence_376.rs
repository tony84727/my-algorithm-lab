pub struct Solution;

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let sign = Self::sign_sequence(nums);
        Self::calculate_longest_wiggle(sign) + 1
    }

    fn sign_sequence(nums: Vec<i32>) -> String {
        nums[0..nums.len() - 1]
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let diff = nums[i + 1] - *n;
                match diff {
                    i if i > 0 => '+',
                    0 => '=',
                    _ => '-',
                }
            })
            .filter(|s| *s != '=')
            .collect()
    }

    fn calculate_longest_wiggle(sign_seq: String) -> i32 {
        if sign_seq.len() <= 1 {
            return 0;
        }
        let mut repeated = 0;
        let mut last = ' ';
        for c in sign_seq.chars() {
            if c == last {
                repeated += 1;
            } else {
                last = c;
            }
        }
        sign_seq.len() as i32 - repeated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    #[test_case(vec![1,7,4,9,2,5] => 6; "example 1")]
    #[test_case(vec![0,0] => 1; "case 1")]
    fn test_solution(input: Vec<i32>) -> i32 {
        Solution::wiggle_max_length(input)
    }
}
