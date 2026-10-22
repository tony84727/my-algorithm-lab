pub struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        Self::solve_at(&questions, 0)
    }

    pub fn solve_at(questions: &[Vec<i32>], i: usize) -> i64 {
        if i >= questions.len() {
            return 0;
        }
        let score = questions[i][0] as i64;
        let cooldown = questions[i][1];
        (score + Self::solve_at(questions, i + 1 + cooldown as usize))
            .max(Self::solve_at(questions, i + 1))
    }
}
