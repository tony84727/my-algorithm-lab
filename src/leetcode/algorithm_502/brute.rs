pub struct Solution;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut projects = profits
            .iter()
            .copied()
            .zip(capital.iter().copied())
            .collect::<Vec<(i32, i32)>>();
        projects.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        let mut total = w;
        let mut capital = w;
        for _ in 0..k {
            match projects
                .iter()
                .enumerate()
                .find(|(_, &(_, requirement))| capital >= requirement)
            {
                Some((i, (profit, _))) => {
                    total += profit;
                    capital += profit;
                    projects.remove(i);
                }
                None => break,
            }
        }
        total
    }
}
