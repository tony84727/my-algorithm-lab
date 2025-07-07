use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable_by_key(|e| e[0]);
        let last_day = events.iter().map(|x| x[1]).max().unwrap();
        let mut end_days: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut attended = 0;
        let mut j = 0;
        for d in 1..=last_day {
            while j < events.len() && events[j][0] <= d {
                end_days.push(Reverse(events[j][1]));
                j += 1;
            }
            while let Some(&Reverse(closest_end)) = end_days.peek() {
                if closest_end < d {
                    end_days.pop();
                } else {
                    break;
                }
            }
            if let Some(_closest_end) = end_days.pop() {
                attended += 1;
            }
        }
        attended
    }
}

#[cfg(test)]
mod tests {
    use crate::vecvec;

    use super::*;
    use test_case::test_case;

    #[test_case(vecvec![[1,2],[2,3],[3,4]] => 3; "example 1")]
    #[test_case(vecvec![[1,2],[2,3],[3,4], [1,2]] => 4; "example 2")]
    #[test_case(vecvec![[1,1],[1,2],[1,3],[1,4],[1,5],[1,6],[1,7]] => 7; "case 1")]
    #[test_case(vecvec![[1,2],[1,2],[3,3],[1,5],[1,5]] => 5; "case 2")]
    #[test_case(vecvec![[1,2],[1,2],[1,2]] => 2; "case 3")]
    #[test_case(vecvec![[25,26],[19,19],[9,13],[16,17],[17,18],[20,21],[22,25],[11,12],[19,23],[7,9],[1,1],[21,23],[14,14],[4,7],[16,16],[24,28],[16,18],[4,5],[18,20],[16,16],[25,26]] => 19; "case 4")]
    #[test_case(vec![vec![1,100000]] => 1; "case 5")]
    #[test_case(vecvec![[52,79],[7,34]] => 2; "case 6")]
    fn test_solution(events: Vec<Vec<i32>>) -> i32 {
        Solution::max_events(events)
    }
}
