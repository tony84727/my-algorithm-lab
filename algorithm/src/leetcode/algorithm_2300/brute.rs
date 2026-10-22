pub struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        spells
            .into_iter()
            .map(|s| {
                potions
                    .iter()
                    .map(|&x| s as i64 * x as i64)
                    .filter(|&x| x >= success)
                    .count() as i32
            })
            .collect()
    }
}
