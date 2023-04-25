use std::{cmp::Reverse, collections::BinaryHeap};

pub struct SmallestInfiniteSet {
    counter: i32,
    pushed: BinaryHeap<Reverse<i32>>,
}

impl SmallestInfiniteSet {
    pub fn new() -> Self {
        Self {
            counter: 1,
            pushed: BinaryHeap::new(),
        }
    }

    pub fn pop_smallest(&mut self) -> i32 {
        if let Some(Reverse(min)) = self.pushed.peek().cloned() {
            match min.cmp(&self.counter) {
                std::cmp::Ordering::Greater => {
                    let temp = self.counter;
                    self.counter += 1;
                    return temp;
                }
                std::cmp::Ordering::Less => {
                    while let Some(Reverse(x)) = self.pushed.peek() {
                        if *x != min {
                            break;
                        }
                        self.pushed.pop();
                    }
                    return min;
                }
                std::cmp::Ordering::Equal => {
                    while let Some(Reverse(x)) = self.pushed.peek() {
                        if *x != min {
                            break;
                        }
                        self.pushed.pop();
                    }
                    self.counter += 1;
                    return min;
                }
            }
        }
        let temp = self.counter;
        self.counter += 1;
        temp
    }

    pub fn add_back(&mut self, num: i32) {
        if self.counter <= num {
            return;
        }
        self.pushed.push(Reverse(num));
    }
}

impl Default for SmallestInfiniteSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut set = SmallestInfiniteSet::new();
        set.add_back(2);
        assert_eq!(1, set.pop_smallest());
        assert_eq!(2, set.pop_smallest());
        assert_eq!(3, set.pop_smallest());
        set.add_back(1);
        assert_eq!(1, set.pop_smallest());
        assert_eq!(4, set.pop_smallest());
        assert_eq!(5, set.pop_smallest());
    }

    #[test]
    fn test_case1() {
        let mut set = SmallestInfiniteSet::new();
        set.add_back(1);
        set.add_back(1);
        set.add_back(1);
        assert_eq!(1, set.pop_smallest());
        assert_eq!(2, set.pop_smallest());
    }
}
