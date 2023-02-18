pub struct Solution;

struct MergeIterator<A: Ord, T: Iterator<Item = A>> {
    a: T,
    b: T,
    a_next: Option<A>,
    b_next: Option<A>,
}

impl<A: Ord, T: Iterator<Item = A>> MergeIterator<A, T> {
    fn new(mut a: T, mut b: T) -> Self {
        let a_next = a.next();
        let b_next = b.next();
        Self {
            a,
            b,
            a_next,
            b_next,
        }
    }
}

impl<A: Ord, T: Iterator<Item = A>> Iterator for MergeIterator<A, T> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        let (advanced, next) = match (&mut self.a_next, &mut self.b_next) {
            (None, None) => return None,
            (a_item @ Some(_), None) => (a_item, self.a.next()),
            (None, b_item @ Some(_)) => (b_item, self.b.next()),
            (a_item @ Some(_), b_item @ Some(_)) => {
                if a_item < b_item {
                    (a_item, self.a.next())
                } else {
                    (b_item, self.b.next())
                }
            }
        };
        std::mem::replace(advanced, next)
    }
}

impl Solution {
    pub fn find_median_sorted_arrays(a: Vec<i32>, b: Vec<i32>) -> f64 {
        let odd = (a.len() + b.len()) % 2 == 1;
        let mut slow = None;
        let mut slow_second = None;
        let mut slow_iter = MergeIterator::new(a.iter(), b.iter());
        let mut fast_iter = MergeIterator::new(a.iter(), b.iter());
        'main: loop {
            slow_second = slow;
            slow = slow_iter.next();
            for _ in 0..2 {
                if fast_iter.next().is_none() {
                    break 'main;
                }
            }
        }
        if odd {
            *slow.unwrap_or(&0) as f64
        } else {
            (slow.unwrap_or(&0) + slow_second.unwrap_or(&0)) as f64 / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,3], vec![2] => 2.0; "example 1")]
    #[test_case(vec![1,2], vec![3,4] => 2.5; "example 2")]
    fn test_solution(a: Vec<i32>, b: Vec<i32>) -> f64 {
        Solution::find_median_sorted_arrays(a, b)
    }
}
