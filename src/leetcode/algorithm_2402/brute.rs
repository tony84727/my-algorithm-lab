use std::{cmp::Ordering, collections::BinaryHeap};

pub struct Solution;

#[derive(Debug)]
struct FreeRooms {
    free_at: Vec<i64>,
}

impl FreeRooms {
    fn new(n: usize) -> Self {
        Self {
            free_at: vec![0; n],
        }
    }
    fn insert(&mut self, free_at: i64, id: usize) {
        self.free_at[id] = free_at;
    }

    // find the free room for free_at, return the lowest room_id
    fn get_free(&mut self, free_at: i64) -> Option<usize> {
        for (i, &room_free_at) in self.free_at.iter().enumerate() {
            if room_free_at <= free_at {
                return Some(i);
            }
        }
        None
    }

    // find the free_at and room with lowest free_at > given free_at
    fn wait_free(&self, free_at: i64) -> (i64, usize) {
        let mut min: Option<(i64, usize)> = None;
        for (i, &room_free_at) in self.free_at.iter().enumerate() {
            if room_free_at <= free_at {
                continue;
            }
            if min.is_none() || min.unwrap().0 > room_free_at {
                min = Some((room_free_at, i));
            }
        }
        min.unwrap()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Meeting {
    start: i64,
    duration: i64,
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
        let mut free_rooms = FreeRooms::new(n as usize);
        let mut todo: BinaryHeap<Meeting> = BinaryHeap::new();
        for m in meetings.into_iter().map(|m| Meeting {
            start: m[0] as i64,
            duration: (m[1] - m[0]) as i64,
        }) {
            todo.push(m);
        }
        let mut current = 0;
        while let Some(m) = todo.pop() {
            current = current.max(m.start);
            if let Some(free_room_id) = free_rooms.get_free(current) {
                booked[free_room_id] += 1;
                free_rooms.insert(current + m.duration, free_room_id);
                continue;
            }
            let (free_at, room) = free_rooms.wait_free(current);
            current = current.max(free_at);
            booked[room] += 1;
            free_rooms.insert(current + m.duration, room);
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
    #[test_case(4, vecvec![[18,19],[3,12],[17,19],[2,13],[7,10]] => 0; "case 1")]
    fn test_solution(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        Solution::most_booked(n, meetings)
    }
}
