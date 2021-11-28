pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .enumerate()
            .map(|(i, _)| {
                nums.iter()
                    .enumerate()
                    .filter(|&(j, _)| i != j)
                    .fold(1, |acc, (_, c)| acc * c)
            })
            .collect()
    }
}
