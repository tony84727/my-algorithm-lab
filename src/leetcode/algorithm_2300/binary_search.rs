pub struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        let length = potions.len();
        potions.sort_unstable();
        spells
            .into_iter()
            .map(|s| {
                let mut left = 0;
                let mut right = length;
                let mut last = length;
                while left < right {
                    let mid = left + (right - left) / 2;
                    let potion = potions[mid];
                    let product = s as i64 * potion as i64;
                    if product >= success {
                        right = mid;
                        last = mid;
                    } else {
                        left = mid + 1;
                    }
                }
                (length - last) as i32
            })
            .collect()
    }
}
