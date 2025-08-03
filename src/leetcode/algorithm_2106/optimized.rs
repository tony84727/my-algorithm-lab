pub struct Solution;

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let mut prefix_sum = vec![0; fruits.len() + 1];
        for (i, f) in fruits.iter().enumerate() {
            prefix_sum[i + 1] = prefix_sum[i] + f[1];
        }
        let mut max = 0;
        let k = k as usize;
        let start_pos = start_pos as usize;
        for go_left in 0..=start_pos.min(k) {
            let left_position = start_pos - go_left;
            let right_position = start_pos + k.saturating_sub(go_left * 2);

            let left = match fruits.binary_search_by(|x| x[0].cmp(&(left_position as i32))) {
                Ok(found) => prefix_sum[found],
                Err(not_exist) => prefix_sum[not_exist],
            };
            let right = match fruits.binary_search_by(|x| x[0].cmp(&(right_position as i32))) {
                Ok(found) => prefix_sum[found + 1],
                Err(not_exist) => prefix_sum[not_exist],
            };
            max = max.max(right - left);
        }
        for go_right in 0..=k {
            let left_position = start_pos.saturating_sub(k.saturating_sub(go_right * 2));
            let right_position = start_pos + go_right;

            let left = match fruits.binary_search_by(|x| x[0].cmp(&(left_position as i32))) {
                Ok(found) => prefix_sum[found],
                Err(not_exist) => prefix_sum[not_exist],
            };
            let right = match fruits.binary_search_by(|x| x[0].cmp(&(right_position as i32))) {
                Ok(found) => prefix_sum[found + 1],
                Err(not_exist) => prefix_sum[not_exist],
            };
            max = max.max(right - left);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[2,8],[6,3],[8,6]], 5, 4 => 9; "example 1")]
    #[test_case(vecvec![[0,9],[4,1],[5,7],[6,2],[7,4],[10,9]], 5, 4 => 14; "example 2")]
    #[test_case(vec![vec![200000,10000]],200000,0=> 10000; "case 1")]
    #[test_case(vec![vec![200000,10000]],0,200000=> 10000; "case 2")]
    #[test_case(vecvec![[0,7],[7,4],[9,10],[12,6],[14,8],[16,5],[17,8],[19,4],[20,1],[21,3],[24,3],[25,3],[26,1],[28,10],[30,9],[31,6],[32,1],[37,5],[40,9]], 21, 30 => 71; "case 3")]
    fn test_solution(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        Solution::max_total_fruits(fruits, start_pos, k)
    }
}
