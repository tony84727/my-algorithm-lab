use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let choices = vec![0; piles.len()];
        let mut dp = vec![HashMap::new(); k as usize];
        Self::solve(&mut dp, choices, &piles, k)
    }

    fn solve(
        dp: &mut Vec<HashMap<Vec<usize>, i32>>,
        choices: Vec<usize>,
        piles: &Vec<Vec<i32>>,
        k: i32,
    ) -> i32 {
        if k == 0 {
            return 0;
        }
        if let Some(answer) = dp[(k - 1) as usize].get(&choices) {
            return *answer;
        }
        let mut max = 0;
        for (i, p) in piles.iter().enumerate() {
            if let Some(p) = p.get(choices[i]) {
                let mut next_choices = choices.clone();
                next_choices[i] += 1;
                max = max.max(p + Self::solve(dp, next_choices, piles, k - 1));
            }
        }
        dp[(k - 1) as usize].insert(choices, max);
        max
    }
}
