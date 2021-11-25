pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = None;
        for start in 0..nums.len() {
            for end in start..nums.len() {
                let sum = nums[start..=end].iter().fold(0, |acc, c| acc + c);
                match max {
                    None => {
                        max = Some(sum);
                    }
                    Some(m) => {
                        if sum > m {
                            max = Some(sum);
                        }
                    }
                }
            }
        }
        max.unwrap_or(0)
    }
}
