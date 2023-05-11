pub struct Solution;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::link_first(&nums1, &nums2)
    }

    fn link_first(nums1: &[i32], nums2: &[i32]) -> i32 {
        if nums1.is_empty() || nums2.is_empty() {
            return 0;
        }
        if nums1.first() == nums2.first() {
            return Self::link_first(&nums1[1..], &nums2[1..]) + 1;
        }
        Self::link_first(nums1, &nums2[1..]).max(Self::link_first(&nums1[1..], nums2))
    }
}
