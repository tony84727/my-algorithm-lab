use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        if events.is_empty() {
            return 0;
        }
        let mut indexed: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
        let mut last_start = 0;
        for e in events.into_iter() {
            if e[0] > last_start {
                last_start = e[0];
            }
            indexed.entry(e[0]).or_default().push(e);
        }
        let mut memoized = HashMap::new();
        Self::optimal_of_day(&mut memoized, &indexed, 0, last_start, k)
    }

    fn optimal_of_day(
        memoized: &mut HashMap<i32, i32>,
        events: &HashMap<i32, Vec<Vec<i32>>>,
        day: i32,
        last: i32,
        k: i32,
    ) -> i32 {
        if let Some(answer) = memoized.get(&day) {
            return *answer;
        }
        if events.is_empty() || k == 0 || day > last {
            return 0;
        }
        let mut optimal = 0;
        if let Some(today) = events.get(&day) {
            for e in today.iter() {
                let value = e[2] + Self::optimal_of_day(memoized, events, e[1] + 1, last, k - 1);
                if value > optimal {
                    optimal = value;
                }
            }
        }

        let no_attend_any = Self::optimal_of_day(memoized, events, day + 1, last, k);
        if no_attend_any > optimal {
            optimal = no_attend_any;
        }
        memoized.insert(day, optimal);
        optimal
    }
}

#[cfg(test)]
mod tests {
    use crate::vecvec;

    use super::*;
    use test_case::test_case;

    #[test_case(vecvec![[1,2,4],[3,4,3],[2,3,1]], 2 => 7; "example 1")]
    #[test_case(vecvec![[1,2,4],[3,4,3],[2,3,10]], 2 => 10; "example 2")]
    fn test_solution(events: Vec<Vec<i32>>, k: i32) -> i32 {
        Solution::max_value(events, k)
    }
}
