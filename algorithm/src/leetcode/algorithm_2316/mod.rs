use std::{cell::RefCell, rc::Rc};

pub struct Solution;

struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    value: i32,
    rank: i32,
    size: i32,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            parent: None,
            value,
            rank: 0,
            size: 1,
        }))
    }

    fn is_root(&self) -> bool {
        self.parent.is_none()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

struct UnionFind {
    sets: Vec<Rc<RefCell<Node>>>,
}

impl UnionFind {
    fn new(size: i32) -> Self {
        let sets = (0..size).map(Node::new).collect();
        Self { sets }
    }
    fn get_set(&self, value: i32) -> Rc<RefCell<Node>> {
        self.sets[value as usize].clone()
    }
    fn find_set(x: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let mut node = x.borrow_mut();
        match &node.parent {
            Some(parent) => {
                let found = Self::find_set(parent.clone());
                node.parent = Some(found.clone());
                found
            }
            None => x.clone(),
        }
    }
    fn union(&self, a: i32, b: i32) {
        let a = self.get_set(a);
        let b = self.get_set(b);
        self.link(Self::find_set(a), Self::find_set(b));
    }

    fn link(&self, a: Rc<RefCell<Node>>, b: Rc<RefCell<Node>>) {
        if a == b {
            return;
        }
        if a.borrow().rank > b.borrow().rank {
            b.borrow_mut().parent = Some(a.clone());
            a.borrow_mut().size += b.borrow().size;
        } else {
            a.borrow_mut().parent = Some(b.clone());
            b.borrow_mut().size += a.borrow().size;
            if a.borrow().rank == b.borrow().rank {
                b.borrow_mut().rank = a.borrow().rank + 1;
            }
        }
    }
    fn component_sizes(self) -> Vec<i64> {
        self.sets
            .into_iter()
            .filter(|node| node.borrow().is_root())
            .map(|x| x.borrow().size as i64)
            .collect()
    }
}

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let union = UnionFind::new(n);
        for edge in edges.into_iter() {
            union.union(edge[0], edge[1]);
        }
        let sizes = union.component_sizes();
        if sizes.len() == 1 {
            return 0;
        }
        let c: i64 = sizes.into_iter().map(|x| (x * x + x) / 2).sum();
        let n = n as i64;
        (n * n + n) / 2 - c
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3, vec![vec![0,1], vec![0,2], vec![1,2]] => 0; "example 1")]
    #[test_case(7, vec![
                vec![0,2],
                vec![0,5],
                vec![2,4],
                vec![1,6],
                vec![5,4],
    ] => 14; "example 2")]
    #[test_case(12, vec![] => 66; "case 1")]
    fn test_solution(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        Solution::count_pairs(n, edges)
    }
}
