#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

pub fn vec_to_list(numbers: Vec<i32>) -> Option<Box<ListNode>> {
    let mut out = None;
    for n in numbers.into_iter().rev() {
        out = Some(Box::new(ListNode { val: n, next: out }))
    }
    out
}

pub fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![];
    let mut node = list;
    while let Some(current) = node {
        out.push(current.val);
        node = current.next;
    }
    out
}
