pub struct Solution;

impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let modulo = 1000000007;
        let mut count = 0;
        let mut min = nums[0];
        let mut max = nums[0];

        if min + max <= target {
            count += 1;
        }

        for &n in nums.iter().skip(1) {
            max = max.max(n);
            min = min.min(n);
            if min + max <= target {
                count *= 2;
                count %= modulo;
            }
        }
        count
    }
}
