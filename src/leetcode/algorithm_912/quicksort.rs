pub struct Solution;

impl Solution {
    fn partition(input: &mut [i32], p: usize, r: usize) -> usize {
        let pivot = input[r - 1];
        let mut i = p;
        for j in p..r - 1 {
            if input[j] <= pivot {
                input.swap(i, j);
                i += 1;
            }
        }
        input.swap(r - 1, i);
        i
    }
    fn quicksort(input: &mut [i32], p: usize, r: usize) {
        if p + 1 < r {
            let q = Self::partition(input, p, r);
            Self::quicksort(input, p, q);
            Self::quicksort(input, q + 1, r);
        }
    }
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        Self::quicksort(&mut nums, 0, len);
        nums
    }
}
