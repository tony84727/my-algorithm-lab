use std::{cmp::Ordering, collections::BinaryHeap};

pub struct Solution;

#[derive(PartialEq, Eq)]
struct Room {
    free_at: i32,
    id: usize,
}

impl PartialOrd for Room {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Room {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let order = other.free_at.cmp(&self.free_at);
        if order != Ordering::Equal {
            return order;
        }
        other.id.cmp(&self.id)
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Meeting {
    start: i32,
    duration: i32,
}

impl PartialOrd for Meeting {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Meeting {
    fn cmp(&self, other: &Self) -> Ordering {
        other.start.cmp(&self.start)
    }
}

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut booked = vec![0; n as usize];
        let mut free_at: BinaryHeap<Room> = BinaryHeap::new();
        for i in 0..n {
            free_at.push(Room {
                free_at: 0,
                id: i as usize,
            });
        }
        let mut todo: BinaryHeap<Meeting> = BinaryHeap::new();
        for m in meetings.into_iter().map(|m| Meeting {
            start: m[0],
            duration: m[1] - m[0],
        }) {
            todo.push(m);
        }
        while let Some(m) = todo.pop() {
            let mut free_room = free_at.peek_mut().unwrap();
            let current = free_room.free_at.max(m.start);
            booked[free_room.id] += 1;
            free_room.free_at = current + m.duration;
            println!(
                "{} @ {} [{current} ~ {}]",
                m.start, free_room.id, free_room.free_at
            );
        }
        let mut max = (0, booked[0]);
        for (i, &v) in booked.iter().enumerate() {
            if v > max.1 {
                max = (i, v);
            }
        }
        max.0 as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::vecvec;

    use super::*;
    use test_case::test_case;

    #[test_case(2,vecvec![[0,10],[1,5],[2,7],[3,4]] => 0; "example 1")]
    #[test_case(3, vecvec![[1,20],[2,10],[3,5],[4,9],[6,8]] => 1; "example 2")]
    // failed case
    // #[test_case(4, vecvec![[18,19],[3,12],[17,19],[2,13],[7,10]] => 0; "case 1")]
    fn test_solution(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        Solution::most_booked(n, meetings)
    }
}
