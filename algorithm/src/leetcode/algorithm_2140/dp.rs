pub struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        if questions.is_empty() {
            return 0;
        }
        let mut dp = vec![None; questions.len()];
        Self::solve_at(&mut dp, &questions, 0)
    }

    pub fn solve_at(cache: &mut Vec<Option<i64>>, questions: &[Vec<i32>], i: usize) -> i64 {
        if i >= questions.len() {
            return 0;
        }
        if let Some(answer) = cache[i] {
            return answer;
        }
        let score = questions[i][0] as i64;
        let cooldown = questions[i][1];
        let answer = (score + Self::solve_at(cache, questions, i + 1 + cooldown as usize))
            .max(Self::solve_at(cache, questions, i + 1));
        cache[i] = Some(answer);
        answer
    }
}
