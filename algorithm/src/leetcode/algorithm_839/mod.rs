use std::{cell::RefCell, rc::Rc};

pub struct Solution;

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let uf = UnionFind::new(strs.len());
        for (i, a) in strs.iter().enumerate() {
            for (j, b) in strs.iter().enumerate() {
                if Self::similar(a, b) {
                    uf.union(i, j);
                }
            }
        }
        uf.component_count() as i32
    }

    fn similar(a: &str, b: &str) -> bool {
        let mut different = 0;
        for (a, b) in a.chars().zip(b.chars()) {
            if a == b {
                continue;
            }
            different += 1;
        }
        different == 0 || different == 2
    }
}

struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    index: usize,
    rank: i32,
}

impl Node {
    fn new(index: usize) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Self {
            parent: None,
            index,
            rank: 1,
        }))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

struct UnionFind {
    sets: Vec<Rc<RefCell<Node>>>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let sets = (0..size).map(Node::new).collect();
        Self { sets }
    }

    fn union(&self, a: usize, b: usize) {
        let a = self.lookup(a);
        let b = self.lookup(b);
        self.link(a, b);
    }

    fn lookup(&self, i: usize) -> Rc<RefCell<Node>> {
        self.sets[i].clone()
    }

    fn link(&self, a: Rc<RefCell<Node>>, b: Rc<RefCell<Node>>) {
        let a = Self::find_parent(a);
        let b = Self::find_parent(b);
        if a == b {
            return;
        }
        if a.borrow().rank > b.borrow().rank {
            b.borrow_mut().parent = Some(a.clone());
            a.borrow_mut().rank += b.borrow().rank;
        } else {
            a.borrow_mut().parent = Some(b.clone());
            b.borrow_mut().rank += a.borrow().rank;
        }
    }

    fn find_parent(a: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        if let Some(parent) = a.borrow_mut().parent.as_mut() {
            let found = Self::find_parent(parent.clone());
            *parent = found.clone();
            return found;
        }
        a
    }

    fn component_count(&self) -> usize {
        self.sets
            .iter()
            .filter(|x| x.borrow().parent.is_none())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&["tars","rats","arts","star"] => 2; "example 1")]
    fn test_solution(strs: &[&'static str]) -> i32 {
        let strs = strs.iter().map(|x| x.to_string()).collect();
        Solution::num_similar_groups(strs)
    }
}
