use std::{cmp::Ordering, collections::BinaryHeap};

pub struct Solution;

#[derive(Eq, PartialEq, Debug)]
struct Class {
    total: i32,
    pass: i32,
}

impl Class {
    fn boost(&self) -> f64 {
        ((self.pass + 1) as f64 / (self.total + 1) as f64) - (self.pass as f64 / self.total as f64)
    }
}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Class {
    fn cmp(&self, other: &Self) -> Ordering {
        self.boost().total_cmp(&other.boost())
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let i = classes.len();
        let mut to_enhance: BinaryHeap<Class> = BinaryHeap::new();
        let mut all_pass = 0;
        for c in classes.into_iter() {
            if c[0] == c[1] {
                all_pass += 1;
                continue;
            }
            to_enhance.push(Class {
                total: c[1],
                pass: c[0],
            });
        }
        for _ in 0..extra_students {
            let Some(Class { total, pass }) = to_enhance.pop() else {
                break;
            };
            to_enhance.push(Class {
                total: total + 1,
                pass: pass + 1,
            });
        }
        (all_pass as f64
            + to_enhance
                .into_iter()
                .map(|Class { total, pass }| pass as f64 / total as f64)
                .sum::<f64>())
            / i as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[1,2],[3,5],[2,2]], 2 => 0.78333; "example 1")]
    #[test_case(vecvec![[2,4],[3,9],[4,5],[2,10]], 4 => 0.53485; "example 2")]
    fn test_solution(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        (Solution::max_average_ratio(classes, extra_students) * 100000.0).round() / 100000.0
    }
}
