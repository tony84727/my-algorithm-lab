pub struct Solution;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut cache = vec![vec![None; nums2.len()]; nums1.len()];
        Self::link_first(&mut cache, &nums1, &nums2, 0, 0)
    }

    fn link_first(
        cache: &mut Vec<Vec<Option<i32>>>,
        nums1: &[i32],
        nums2: &[i32],
        start1: usize,
        start2: usize,
    ) -> i32 {
        if start1 == nums1.len() || start2 == nums2.len() {
            return 0;
        }
        if let Some(answer) = cache[start1][start2] {
            return answer;
        }
        if nums1[start1] == nums2[start2] {
            let answer = Self::link_first(cache, nums1, nums2, start1 + 1, start2 + 1) + 1;
            cache[start1][start2] = Some(answer);
            return answer;
        }
        let answer = Self::link_first(cache, nums1, nums2, start1, start2 + 1)
            .max(Self::link_first(cache, nums1, nums2, start1 + 1, start2));
        cache[start1][start2] = Some(answer);
        answer
    }
}
