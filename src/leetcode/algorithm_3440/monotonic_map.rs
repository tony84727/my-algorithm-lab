use std::collections::BTreeMap;

pub struct Solution;

struct Tree<D>
where
    D: Fn(usize, usize) -> bool,
{
    dominate: D,
    index: BTreeMap<i32, usize>,
}

impl<D> Tree<D>
where
    D: Fn(usize, usize) -> bool,
{
    fn new(dominate: D) -> Self {
        Self {
            dominate,
            index: BTreeMap::new(),
        }
    }
    fn insert(&mut self, gap_size: i32, position: usize) {
        if let Some((_, existing)) = self.index.range(gap_size..).next() {
            if (self.dominate)(*existing, position) {
                return;
            }
        }
        let to_remove: Vec<i32> = self
            .index
            .range(..gap_size)
            .take_while(|(_, &v)| (self.dominate)(position, v))
            .map(|(&k, _)| k)
            .collect();
        for k in to_remove.into_iter() {
            self.index.remove(&k);
        }
        self.index.insert(gap_size, position);
    }

    fn query(&self, gap_size: i32) -> Option<usize> {
        self.index
            .range(gap_size..)
            .next()
            .map(|(_, &position)| position)
    }
}

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut move_right_index = Tree::new(|a, b| a >= b);
        let mut move_left_index = Tree::new(|a, b| a <= b);
        let n = start_time.len();
        let mut max_gap = 0;
        for i in 0..=n {
            let gap_start = if i == 0 { 0 } else { end_time[i - 1] };
            let gap_end = if i == n { event_time } else { start_time[i] };
            let gap_size = gap_end - gap_start;
            max_gap = max_gap.max(gap_size);
            move_right_index.insert(gap_size, i);
            move_left_index.insert(gap_size, i);
        }
        for i in 0..n {
            let gap_start = if i == 0 { 0 } else { end_time[i - 1] };
            let next_gap_end = if i + 1 == n {
                event_time
            } else {
                start_time[i + 1]
            };
            let duration = end_time[i] - start_time[i];
            let mut rescheduled_gap = next_gap_end - gap_start - duration;
            if Self::can_reschedule_right(i, &move_right_index, duration)
                || Self::can_reschedule_left(i, &move_left_index, duration)
            {
                rescheduled_gap += duration;
            }
            if rescheduled_gap > max_gap {
                max_gap = max_gap.max(rescheduled_gap);
            }
        }
        max_gap
    }

    fn can_reschedule_right<D>(event: usize, index: &Tree<D>, duration: i32) -> bool
    where
        D: Fn(usize, usize) -> bool,
    {
        matches!(index.query(duration), Some(p) if p > event + 1)
    }

    fn can_reschedule_left<D>(event: usize, index: &Tree<D>, duration: i32) -> bool
    where
        D: Fn(usize, usize) -> bool,
    {
        matches!(index.query(duration), Some(p) if p < event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5, vec![1,3], vec![2,5] => 2; "example 1")]
    #[test_case(10, vec![0,7,9], vec![1,8,10] => 7; "example 2")]
    #[test_case(10, vec![0,3,7,9], vec![1,4,8,10] => 6; "example 3")]
    #[test_case(86, vec![22,82], vec![66,85] => 38; "case 1")]
    #[test_case(37, vec![5,14,27,34], vec![13,18,31,37] => 16; "case 2")]
    #[test_case(54, vec![9,24,45,50,53], vec![15,26,50,53,54] => 30; "case 3")]
    #[test_case(158, vec![15,30,36,42,46,146], vec![30,31,42,44,96,152] => 62; "case 4")]
    fn test_solution(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        Solution::max_free_time(event_time, start_time, end_time)
    }
}
