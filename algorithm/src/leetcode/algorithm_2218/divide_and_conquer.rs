pub struct Solution;

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let choices = vec![0; piles.len()];
        Self::solve(&choices, &piles, k)
    }

    fn solve(choices: &[usize], piles: &Vec<Vec<i32>>, k: i32) -> i32 {
        if k == 0 {
            return 0;
        }
        let mut max = 0;
        for (i, p) in piles.iter().enumerate() {
            if let Some(p) = p.get(choices[i]) {
                let mut next_choices = choices.to_vec();
                next_choices[i] += 1;
                max = max.max(p + Self::solve(&next_choices, piles, k - 1));
            }
        }
        max
    }
}
