use std::cmp::Ordering;

pub struct Solution;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Event {
    start: i32,
    end: i32,
}

impl Event {
    fn is_valid(&self, day: i32) -> bool {
        self.start <= day && self.end >= day
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.start.cmp(&other.start) {
            Ordering::Equal => self.end.cmp(&other.end),
            ord => ord,
        }
    }
}

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut last_day = 0;
        for e in events.iter() {
            if e[1] > last_day {
                last_day = e[1]
            }
        }
        let events = Solution::sort_events(events);
        Self::attended_after_day(events, 0, last_day)
    }

    fn attended_after_day(events: Vec<Event>, day: i32, max: i32) -> i32 {
        if events.is_empty() {
            return 0;
        }
        let mut optimal = 0;
        for d in day..=max {
            for (i, e) in events.iter().enumerate() {
                if e.is_valid(d) {
                    let mut remaining = events.clone();
                    remaining.remove(i);
                    let attended = Self::attended_after_day(remaining, d + 1, max);
                    if attended + 1 > optimal {
                        optimal = attended + 1;
                    }
                }
            }
        }
        optimal
    }

    fn sort_events(events: Vec<Vec<i32>>) -> Vec<Event> {
        let mut events: Vec<Event> = events
            .into_iter()
            .map(|x| Event {
                start: x[0],
                end: x[1],
            })
            .collect();
        events.sort();
        events
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
    fn test_solution(events: Vec<Vec<i32>>) -> i32 {
        Solution::max_events(events)
    }
}
