pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable();
        let mut start = intervals[0][0];
        let mut end = intervals[0][1];
        let mut answers = vec![];

        for n in intervals.into_iter() {
            if end >= n[0] {
                end = n[1];
            } else {
                answers.push(vec![start, end]);
                start = n[0];
                end = n[1];
            }
        }
        answers.push(vec![start, end]);
        answers
    }
}
