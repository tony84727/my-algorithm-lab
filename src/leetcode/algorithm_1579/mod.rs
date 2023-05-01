use std::{cell::RefCell, rc::Rc};

pub struct Solution;

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let a = UnionFind::new(n);
        let b = UnionFind::new(n);
        let mut duplicated = 0;
        for edge in edges.iter() {
            let edge_type = edge[0];
            if edge_type != 3 {
                continue;
            }
            let x = edge[1];
            let y = edge[2];
            let a = a.union(x, y);
            let b = b.union(x, y);
            if !a && !b {
                duplicated += 1;
            }
        }
        for edge in edges.iter() {
            let edge_type = edge[0];
            if !match edge_type {
                1 => &a,
                2 => &b,
                _ => {
                    continue;
                }
            }
            .union(edge[1], edge[2])
            {
                duplicated += 1;
            }
        }
        if a.component_size() > 1 || b.component_size() > 1 {
            return -1;
        }
        duplicated
    }
}

struct Node {
    value: i32,
    rank: i32,
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Self {
            value,
            rank: 1,
            parent: None,
        }))
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
        let sets = (0..size).into_iter().map(Node::new).collect();
        Self { sets }
    }

    fn lookup(&self, value: i32) -> Rc<RefCell<Node>> {
        self.sets[value as usize - 1].clone()
    }

    fn union(&self, a: i32, b: i32) -> bool {
        let a = self.find(self.lookup(a));
        let b = self.find(self.lookup(b));
        if a == b {
            return false;
        }
        if a.borrow().rank > b.borrow().rank {
            b.borrow_mut().parent = Some(a);
        } else {
            a.borrow_mut().parent = Some(b.clone());
            if a.borrow().rank == b.borrow().rank {
                b.borrow_mut().rank = a.borrow().rank + 1;
            }
        }
        true
    }

    fn find(&self, x: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let mut node = x.borrow_mut();
        match &node.parent {
            Some(parent) => {
                let found = self.find(parent.clone());
                node.parent = Some(found.clone());
                found
            }
            None => x.clone(),
        }
    }

    fn component_size(&self) -> usize {
        let count = self
            .sets
            .iter()
            .filter(|root| root.borrow().parent.is_none())
            .count();
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(4, vec![(3,1,2),(3,2,3),(1,1,3),(1,2,4),(1,1,2),(2,3,4)] => 2)]
    fn test_solution(n: i32, edges: Vec<(i32, i32, i32)>) -> i32 {
        Solution::max_num_edges_to_remove(
            n,
            edges
                .into_iter()
                .map(|(edge_type, a, b)| vec![edge_type, a, b])
                .collect(),
        )
    }
}
