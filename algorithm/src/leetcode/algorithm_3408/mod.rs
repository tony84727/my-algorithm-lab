use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Eq, PartialEq)]
struct Entry {
    task: i32,
    priority: i32,
    version: usize,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let priority_cmp = self.priority.cmp(&other.priority);
        if priority_cmp != Ordering::Equal {
            return priority_cmp;
        }
        self.task.cmp(&other.task)
    }
}

#[derive(Default)]
pub struct TaskManager {
    tasks: BinaryHeap<Entry>,
    versions: HashMap<i32, usize>,
    task_owners: HashMap<i32, i32>,
}

impl TaskManager {
    pub fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut manager = TaskManager::default();
        for t in tasks.into_iter() {
            let user = t[0];
            let task = t[1];
            let priority = t[2];
            manager.add(user, task, priority);
        }
        manager
    }

    pub fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        let version = self.versions.entry(task_id).or_default();
        *version += 1;
        self.task_owners.insert(task_id, user_id);
        self.tasks.push(Entry {
            task: task_id,
            priority,
            version: *version,
        });
    }

    pub fn edit(&mut self, task_id: i32, new_priority: i32) {
        let version = self.versions.entry(task_id).or_default();
        *version += 1;
        self.tasks.push(Entry {
            task: task_id,
            priority: new_priority,
            version: *version,
        });
    }

    pub fn rmv(&mut self, task_id: i32) {
        self.task_owners.remove(&task_id);
    }

    pub fn exec_top(&mut self) -> i32 {
        while let Some(Entry { task, version, .. }) = self.tasks.pop() {
            let current_version = match self.versions.get(&task) {
                Some(v) => v,
                None => {
                    continue;
                }
            };
            if &version != current_version {
                continue;
            }
            let Some(user) = self.task_owners.remove(&task) else {
                continue;
            };
            return user;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;

    #[test]
    fn test_solution_example_1() {
        let mut manager = TaskManager::new(vecvec![[1, 101, 10], [2, 102, 20], [3, 103, 15]]);
        manager.add(4, 104, 5);
        manager.edit(102, 8);
        assert_eq!(3, manager.exec_top());
        manager.rmv(101);
        manager.add(5, 105, 15);
        assert_eq!(5, manager.exec_top());
    }

    #[test]
    fn test_solution_case_1() {
        let mut manager = TaskManager::new(vecvec![
            [10, 4, 10],
            [10, 0, 6],
            [5, 23, 50],
            [3, 29, 50],
            [2, 15, 9]
        ]);
        assert_eq!(3, manager.exec_top());
    }

    #[test]
    fn test_solution_case_2() {
        let mut manager = TaskManager::new(vec![vec![10, 26, 25]]);
        manager.rmv(26);
        assert_eq!(-1, manager.exec_top());
    }

    #[test]
    fn test_solution_case_3() {
        let mut manager = TaskManager::new(vecvec![[1, 101, 8], [2, 102, 20], [3, 103, 5]]);
        manager.add(4, 104, 5);
        manager.edit(102, 9);
        assert_eq!(2, manager.exec_top());
        manager.rmv(101);
        manager.add(50, 101, 8);
        assert_eq!(50, manager.exec_top());
    }

    #[test]
    fn test_solution_case_4() {
        let mut manager = TaskManager::new(vec![vec![8, 21, 43]]);
        manager.rmv(21);
        manager.add(6, 15, 38);
        manager.rmv(15);
        manager.add(3, 15, 23);
        assert_eq!(3, manager.exec_top());
        assert_eq!(-1, manager.exec_top());
    }

    #[test]
    fn test_solution_case_5() {
        let mut manager = TaskManager::new(vecvec![
            [6, 25, 15],
            [7, 20, 1],
            [5, 5, 40],
            [2, 19, 47],
            [4, 14, 45]
        ]);
        manager.add(6, 17, 40);
        manager.rmv(20);
        manager.add(6, 1, 0);
        assert_eq!(2, manager.exec_top());
        manager.rmv(14);
        manager.add(5, 13, 35);
        manager.rmv(1);
        manager.add(4, 4, 14);
        manager.rmv(17);
        manager.add(1, 17, 37);
        manager.edit(4, 40);
        manager.rmv(4);
        manager.add(1, 8, 21);
        assert_eq!(5, manager.exec_top());
    }
}
