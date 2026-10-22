pub struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        let length = potions.len();
        potions.sort_unstable();
        spells
            .into_iter()
            .map(|s| {
                potions
                    .iter()
                    .enumerate()
                    .find_map(|(i, &x)| {
                        if x as i64 * s as i64 >= success {
                            Some(length - i)
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default() as i32
            })
            .collect()
    }
}
