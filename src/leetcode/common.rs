use std::{cell::RefCell, rc::Rc};
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

#[cfg(test)]
pub fn vec_to_list(numbers: Vec<i32>) -> Option<Box<ListNode>> {
    let mut out = None;
    for n in numbers.into_iter().rev() {
        out = Some(Box::new(ListNode { val: n, next: out }))
    }
    out
}

#[cfg(test)]
pub fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![];
    let mut node = list;
    while let Some(current) = node {
        out.push(current.val);
        node = current.next;
    }
    out
}

#[cfg(test)]
pub mod test_utils {
    use serde::de::DeserializeOwned;
    use std::{fs::File, path::Path};

    pub fn from_file<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> T {
        let mut f = File::open(path).unwrap();
        ron::de::from_reader(&mut f).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
