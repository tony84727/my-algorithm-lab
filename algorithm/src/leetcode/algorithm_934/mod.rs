use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, columns) = Self::get_dimension(&grid);
        let islands = UnionFind::new(rows, columns);
        let mut first_land = None;
        for (i, row) in grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    continue;
                }
                if first_land.is_none() {
                    first_land = Some((i, j));
                }
                if i > 0 && grid[i - 1][j] == 1 {
                    islands.union((i - 1, j), (i, j));
                }
                if i < rows - 1 && grid[i + 1][j] == 1 {
                    islands.union((i + 1, j), (i, j));
                }
                if j > 0 && grid[i][j - 1] == 1 {
                    islands.union((i, j - 1), (i, j));
                }
                if j < columns - 1 && grid[i][j + 1] == 1 {
                    islands.union((i, j + 1), (i, j));
                }
            }
        }
        let first_island = UnionFind::find_set(islands.get_set(first_land.unwrap()));
        let mut to_visit = vec![];
        for i in 0..rows {
            for j in 0..columns {
                if UnionFind::find_set(islands.get_set((i, j))) == first_island {
                    to_visit.push((i, j));
                }
            }
        }
        let mut length = 0;
        while !to_visit.is_empty() {
            let to_walk = std::mem::take(&mut to_visit);
            for (i, j) in to_walk {
                if i > 0 && islands.union((i - 1, j), (i, j)) {
                    if grid[i - 1][j] == 1 {
                        return length;
                    }
                    to_visit.push((i - 1, j));
                }
                if i < rows - 1 && islands.union((i + 1, j), (i, j)) {
                    if grid[i + 1][j] == 1 {
                        return length;
                    }
                    to_visit.push((i + 1, j));
                }
                if j > 0 && islands.union((i, j - 1), (i, j)) {
                    if grid[i][j - 1] == 1 {
                        return length;
                    }
                    to_visit.push((i, j - 1));
                }
                if j < columns - 1 && islands.union((i, j + 1), (i, j)) {
                    if grid[i][j + 1] == 1 {
                        return length;
                    }
                    to_visit.push((i, j + 1));
                }
            }
            length += 1;
        }
        0
    }

    fn get_dimension(grid: &[Vec<i32>]) -> (usize, usize) {
        (
            grid.len(),
            grid.first().map(|x| x.len()).unwrap_or_default(),
        )
    }
}

struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    value: i32,
    rank: i32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            parent: None,
            rank: 0,
            value,
        }
    }
}

struct UnionFind {
    grid: Vec<Vec<Rc<RefCell<Node>>>>,
}

impl UnionFind {
    fn new(rows: usize, columns: usize) -> Self {
        let mut grid: Vec<Vec<Rc<RefCell<Node>>>> = Vec::with_capacity(rows);
        let mut count = 0;
        for i in 0..rows {
            grid.push(Vec::with_capacity(columns));
            for _j in 0..columns {
                grid[i].push(Rc::new(RefCell::new(Node::new(count))));
                count += 1;
            }
        }
        Self { grid }
    }
    fn get_set(&self, position: (usize, usize)) -> Rc<RefCell<Node>> {
        let (i, j) = position;
        self.grid[i][j].clone()
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
    fn union(&self, a: (usize, usize), b: (usize, usize)) -> bool {
        let a = self.get_set(a);
        let b = self.get_set(b);
        self.link(Self::find_set(a), Self::find_set(b))
    }

    fn link(&self, a: Rc<RefCell<Node>>, b: Rc<RefCell<Node>>) -> bool {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![vec![0,1],vec![1,0]] => 1; "example 1")]
    #[test_case(vec![vec![0,1,0],vec![0,0,0],vec![0,0,1]] => 2; "example 2")]
    #[test_case(vec![
                vec![1,1,1,1,1],
                vec![1,0,0,0,1],
                vec![1,0,1,0,1],
                vec![1,0,0,0,1],
                vec![1,1,1,1,1],
    ] => 1; "example 3")]
    #[test_case(vec![
                vec![1,1,0,0,0],
                vec![1,0,0,0,0],
                vec![1,0,0,0,0],
                vec![0,0,0,1,1],
                vec![0,0,0,1,1],
    ] => 3; "case 1")]
    #[test_case(vec![
                vec![0,0,1,0,1],
                vec![0,1,1,0,1],
                vec![0,1,0,0,1],
                vec![0,0,0,0,0],
                vec![0,0,0,0,0],
    ] => 1; "case 2")]
    fn test_solution(grid: Vec<Vec<i32>>) -> i32 {
        Solution::shortest_bridge(grid)
    }
}
