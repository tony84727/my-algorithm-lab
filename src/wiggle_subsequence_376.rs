pub struct Solution;

#[derive(PartialEq)]
enum Sign {
    Plus,
    Equal,
   Minus, 
}

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let sign = Self::sign_sequence(nums);
        return Self::calculate_longest_wiggle(sign) + 1;
    }

    fn sign_sequence(nums: Vec<i32>) -> Vec<Sign> {
        nums[0..nums.len()-1].iter().enumerate().map(|(i, n)| {
            let diff = nums[i+1] - *n;
            if diff > 0 {
                Sign::Plus
            } else if diff == 0 {
                Sign::Equal
            } else {
                Sign::Minus
            }
        }
        ).filter(|s| *s == Sign::Equal).collect()
    }

    fn calculate_longest_wiggle(sign_seq: Vec<Sign>) -> i32 {
        let l = sign_seq.len();
        if l <= 1 {
            return 0;
        }
        let mut repeated = 0;
        let mut last = None;
        for c in sign_seq.into_iter() {
            if let Some(last) = &last {
                if c == *last {
                    repeated += 1;
                    continue;
                }
            }
            last = Some(c);
        }
        l as i32 - repeated
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