use std::{collections::HashMap, rc::Rc};

pub struct Solution;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Event {
    start: i32,
    end: i32,
}

impl Event {
    fn is_valid(&self, day: i32) -> bool {
        self.start <= day && self.end >= day
    }
}

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let events: Vec<Event> = events
            .into_iter()
            .map(|e| Event {
                start: e[0],
                end: e[1],
            })
            .collect();
        let mut memoized: HashMap<(Rc<Vec<Event>>, i32), i32> = HashMap::new();
        Self::attended_after_day(&mut memoized, Rc::new(events), 1)
    }

    fn attended_after_day(
        memoized: &mut HashMap<(Rc<Vec<Event>>, i32), i32>,
        events: Rc<Vec<Event>>,
        day: i32,
    ) -> i32 {
        if let Some(answer) = memoized.get(&(events.clone(), day)) {
            return *answer;
        }
        if events.is_empty() {
            memoized.insert((events, day), 0);
            return 0;
        }
        if events.len() == 1 {
            return if events.first().unwrap().end >= day {
                1
            } else {
                0
            };
        }
        let mut optimal = 0;
        let valid: Vec<Event> = events.iter().filter(|x| x.end >= day).cloned().collect();
        let events = Rc::new(valid);
        for (i, e) in events.iter().enumerate() {
            if !e.is_valid(day) {
                continue;
            }
            let mut remaining_p = events.clone();
            Rc::make_mut(&mut remaining_p).remove(i);
            let attended = Self::attended_after_day(memoized, remaining_p, day + 1) + 1;
            if attended > optimal {
                optimal = attended;
            }
        }
        let no_attend = Self::attended_after_day(memoized, events.clone(), day + 1);
        if no_attend > optimal {
            optimal = no_attend
        }
        memoized.insert((events, day), optimal);
        optimal
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
