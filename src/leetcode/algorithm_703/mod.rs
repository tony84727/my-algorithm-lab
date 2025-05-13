use std::cmp::Reverse;

pub struct KthLargest {
    largests: Vec<i32>,
    k: usize,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut largests = nums;
        largests.sort_unstable_by_key(|x| Reverse(*x));
        let k = k as usize;
        if largests.len() > k {
            largests.resize(self.k, 0);
        }
        Self { largests, k }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.largests.push(val);
        self.largests.sort_unstable_by_key(|x| Reverse(*x));
        if self.largests.len() > self.k {
            self.largests.resize(self.k, 0);
        }
        *self.largests.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn test_solution() {
        let mut largest = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(4, largest.add(3));
        assert_eq!(5, largest.add(5));
        assert_eq!(5, largest.add(10));
        assert_eq!(8, largest.add(9));
        assert_eq!(8, largest.add(4));
    }
}
