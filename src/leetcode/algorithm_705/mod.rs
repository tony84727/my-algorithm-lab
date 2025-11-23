use std::collections::HashSet;

pub struct MyHashSet {
    inner: HashSet<i32>,
}

impl MyHashSet {
    pub fn new() -> Self {
        Self {
            inner: HashSet::new(),
        }
    }

    pub fn add(&mut self, key: i32) {
        self.inner.insert(key);
    }

    pub fn remove(&mut self, key: i32) {
        self.inner.remove(&key);
    }

    pub fn contains(&self, key: i32) -> bool {
        self.inner.contains(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::MyHashSet;

    #[test]
    fn test_solution() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(2);
        assert!(set.contains(1));
        assert!(!set.contains(3));
        set.add(2);
        assert!(set.contains(2));
        set.remove(2);
        assert!(!set.contains(2));
    }
}
