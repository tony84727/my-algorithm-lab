pub struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}

impl FindSumPairs {
    pub fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        Self { nums1, nums2 }
    }

    pub fn add(&mut self, index: i32, val: i32) {
        self.nums2[index as usize] += val;
    }

    pub fn count(&self, tot: i32) -> i32 {
        let mut count = 0;
        for a in self.nums1.iter() {
            for b in self.nums2.iter() {
                if a + b == tot {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_example1() {
        let mut find = FindSumPairs::new(vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]);
        assert_eq!(8, find.count(7));
        find.add(3, 2);
        assert_eq!(2, find.count(8));
        assert_eq!(1, find.count(4));
        find.add(0, 1);
        find.add(1, 1);
        assert_eq!(11, find.count(7));
    }
}
