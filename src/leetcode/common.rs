use std::{cell::RefCell, collections::VecDeque, ops::Deref, rc::Rc};
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

fn parse_input(preorder: Vec<&str>) -> Vec<Option<i32>> {
    preorder
        .into_iter()
        .map(|input| {
            if input == "null" {
                return None;
            }
            Some(input.parse().unwrap())
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Default)]
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

    pub fn from_preorder(preorder: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(
            preorder.first().unwrap().expect("no leading nil"),
        )));
        let mut parents = VecDeque::new();
        parents.push_back(Some(root.clone()));
        let mut i = 1;
        while !parents.is_empty() && i < preorder.len() {
            let current = parents.pop_front().unwrap();
            if let Some(current) = current {
                let left = match preorder[i] {
                    Some(value) => {
                        let new_node = Rc::new(RefCell::new(TreeNode::new(value)));
                        current.deref().borrow_mut().left = Some(new_node.clone());
                        Some(new_node)
                    }
                    None => None,
                };
                parents.push_back(left);
                i += 1;
                if i >= preorder.len() {
                    break;
                }
                let right = match preorder[i] {
                    Some(value) => {
                        let new_node = Rc::new(RefCell::new(TreeNode::new(value)));
                        current.deref().borrow_mut().right = Some(new_node.clone());
                        Some(new_node)
                    }
                    None => None,
                };
                parents.push_back(right);
                i += 1;
            }
        }
        Some(root)
    }

    pub fn from_preorder_str(elements: Vec<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::from_preorder(parse_input(elements))
    }
}
