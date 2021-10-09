pub struct Node<T>
where
    T: PartialOrd,
{
    value: T,
    right: Option<Box<Node<T>>>,
    left: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: PartialOrd,
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            right: None,
            left: None,
        }
    }
    pub fn create(elements: Vec<T>) -> Option<Self> {
        let mut root: Option<Self> = None;
        for e in elements.into_iter() {
            match &mut root {
                Some(root) => root.insert(e),
                None => root = Some(Self::new(e)),
            }
        }
        root
    }
    pub fn insert(&mut self, value: T) {
        let mut current = self;
        loop {
            if &current.value == &value {
                return;
            }
            let side = if value < current.value {
                &mut current.left
            } else {
                &mut current.right
            };
            match side {
                Some(next) => {
                    current = next;
                }
                None => {
                    *side = Some(Box::new(Node {
                        value,
                        right: None,
                        left: None,
                    }));
                    return;
                }
            };
        }
    }
    pub fn search(&self, target: &T) -> bool {
        let mut current = Some(self);
        while let Some(node) = current {
            if &node.value == target {
                return true;
            }
            current = if target < &node.value {
                node.left.as_deref()
            } else {
                node.right.as_deref()
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_integers() {
        let numbers = vec![0, 1, 2, 3, 4, 5];
        let root = Node::create(numbers.clone()).unwrap();
        for n in numbers.iter() {
            assert!(root.search(n));
        }
        assert!(!root.search(&6));
    }

    #[test]
    fn test_insert_strs() {
        let strs = vec!["apple", "banana", "cocoa"];
        let root = Node::create(strs.clone()).unwrap();
        for s in strs.iter() {
            assert!(root.search(s));
        }
        assert!(!root.search(&"hello"));
    }
}
