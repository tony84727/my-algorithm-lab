pub struct Solution;

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let n = energy.len();
        let mut max_starting_from = vec![0; n];
        for i in (0..n).rev() {
            let next = i + (k as usize);
            max_starting_from[i] = energy[i]
                + if next >= n {
                    0
                } else {
                    max_starting_from[next]
                };
        }
        max_starting_from.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![5,2,-10,-5,1], 3 => 3; "example 1")]
    #[test_case(vec![-2,-3,-1], 2 => -1; "example 2")]
    #[test_case(vec![5,-10,4,3,5,-9,9,-7], 2 => 23; "case 1")]
    fn test_solution(energy: Vec<i32>, k: i32) -> i32 {
        Solution::maximum_energy(energy, k)
    }
}
