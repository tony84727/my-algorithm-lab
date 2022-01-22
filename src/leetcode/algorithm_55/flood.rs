pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut components: Vec<bool> = vec![false; nums.len()];
        let max = nums.len() - 1;
        for (i, n) in nums.into_iter().enumerate() {
            if i != 0 && !components[i] {
                continue;
            }
            let n = n as usize;
            for j in i..=(i + n).min(max) {
                components[j] = true;
            }
        }
        components[max]
    }
}
