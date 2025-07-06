use std::collections::HashMap;

pub struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    index: HashMap<i32, i32>,
}

impl FindSumPairs {
    pub fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut index = HashMap::new();

        for n in nums2.iter() {
            *index.entry(*n).or_default() += 1;
        }

        Self {
            nums1,
            nums2,
            index,
        }
    }

    pub fn add(&mut self, index: i32, val: i32) {
        let from = self.nums2[index as usize];
        let to = from + val;
        self.nums2[index as usize] = to;
        *self.index.entry(from).or_default() -= 1;
        *self.index.entry(to).or_default() += 1;
    }

    pub fn count(&self, tot: i32) -> i32 {
        let mut count = 0;
        for b in self.nums1.iter() {
            let paired = tot - b;
            count += self.index.get(&paired).cloned().unwrap_or_default();
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
