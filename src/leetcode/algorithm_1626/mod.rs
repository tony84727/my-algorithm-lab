struct Solution;

#[derive(Debug)]
struct Player {
    age: i32,
    score: i32,
}

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut players = scores
            .into_iter()
            .zip(ages.into_iter())
            .map(|(score, age)| Player { age, score })
            .collect::<Vec<Player>>();
        players.sort_unstable_by(|a, b| a.age.cmp(&b.age));
        let mut max = 0;
        let mut with_current = 0;
        let mut without_current = 0;
        for p in players.iter() {
            with_current = with_current + p.score;
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,3,5,10,15], vec![1,2,3,4,5] => 34; "example 1")]
    fn test_solution(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        Solution::best_team_score(scores, ages)
    }
}
