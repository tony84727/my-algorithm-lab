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
            for connected in components.iter_mut().take((i + n).min(max) + 1).skip(i) {
                *connected = true;
            }
        }
        components[max]
    }
}
