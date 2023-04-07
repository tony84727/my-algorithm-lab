use std::{cell::RefCell, rc::Rc};

pub struct Solution;

struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    value: usize,
    rank: i32,
    walled: bool,
    size: usize,
}

impl Node {
    fn new(value: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            parent: None,
            value,
            rank: 0,
            walled: false,
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
    fn new(size: usize) -> Self {
        let sets = (0..size).map(Node::new).collect();
        Self { sets }
    }
    fn get_set(&self, value: usize) -> Rc<RefCell<Node>> {
        self.sets[value].clone()
    }
    fn find_set(&self, x: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let mut node = x.borrow_mut();
        match &node.parent {
            Some(parent) => {
                let found = self.find_set(parent.clone());
                node.parent = Some(found.clone());
                found
            }
            None => x.clone(),
        }
    }
    fn union(&self, a: usize, b: usize) {
        let a = self.get_set(a);
        let b = self.get_set(b);
        self.link(self.find_set(a), self.find_set(b));
    }

    fn link(&self, a: Rc<RefCell<Node>>, b: Rc<RefCell<Node>>) {
        if a == b {
            return;
        }
        if a.borrow().rank > b.borrow().rank {
            b.borrow_mut().parent = Some(a.clone());
            a.borrow_mut().walled |= b.borrow().walled;
            a.borrow_mut().size += b.borrow_mut().size;
        } else {
            a.borrow_mut().parent = Some(b.clone());
            if a.borrow().rank == b.borrow().rank {
                b.borrow_mut().rank = a.borrow().rank + 1;
            }
            b.borrow_mut().walled |= a.borrow().walled;
            b.borrow_mut().size += a.borrow_mut().size;
        }
    }
    fn enclosed_island_count(self) -> usize {
        self.sets
            .into_iter()
            .filter(|node| node.borrow().is_root())
            .filter(|node| !node.borrow().walled)
            .map(|node| node.borrow().size)
            .sum()
    }
}

struct UnionFindGrid {
    find: UnionFind,
    columns: usize,
}

impl UnionFindGrid {
    fn new(rows: usize, columns: usize) -> Self {
        let instance = Self {
            find: UnionFind::new(rows * columns),
            columns,
        };
        for i in 0..columns {
            instance.set_wall((0, i));
            instance.set_wall((rows - 1, i));
        }
        for j in 0..rows {
            instance.set_wall((j, 0));
            instance.set_wall((j, columns - 1));
        }
        instance
    }
    fn union(&self, a: (usize, usize), b: (usize, usize)) {
        self.find.union(self.to_index(a), self.to_index(b))
    }

    fn to_index(&self, a: (usize, usize)) -> usize {
        let (i, j) = a;
        i * self.columns + j
    }

    fn set_wall(&self, a: (usize, usize)) {
        self.find.sets[self.to_index(a)].borrow_mut().walled = true;
    }

    fn enclosed_island_size(self) -> usize {
        self.find.enclosed_island_count()
    }
}

impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let columns = grid.first().unwrap().len();
        let find = UnionFindGrid::new(rows, columns);
        for (i, row) in grid.iter().enumerate() {
            for (j, &column) in row.iter().enumerate() {
                if column == 0 {
                    find.set_wall((i, j));
                    continue;
                }
                if i > 0 && grid[i - 1][j] == 1 {
                    find.union((i, j), (i - 1, j));
                }
                if i < rows - 1 && grid[i + 1][j] == 1 {
                    find.union((i, j), (i + 1, j));
                }
                if j > 0 && grid[i][j - 1] == 1 {
                    find.union((i, j), (i, j - 1));
                }
                if j < columns - 1 && grid[i][j + 1] == 1 {
                    find.union((i, j), (i, j + 1));
                }
            }
        }
        find.enclosed_island_size() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![
                vec![0,0,0,0],
                vec![1,0,1,0],
                vec![0,1,1,0],
                vec![0,0,0,0],
    ] => 3; "example 1")]
    fn test_solution(grid: Vec<Vec<i32>>) -> i32 {
        Solution::num_enclaves(grid)
    }
}
