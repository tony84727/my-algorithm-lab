pub struct Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let digit_sums: Vec<i32> = nums
            .iter()
            .map(|n| {
                let mut current = *n;
                let mut sum = 0;
                while current > 0 {
                    sum += current % 10;
                    current /= 10;
                }
                sum
            })
            .collect();
        let sorted_index = {
            let mut sorting: Vec<usize> = (0..n).collect();
            sorting.sort_by(|a, b| {
                digit_sums[*a]
                    .cmp(&digit_sums[*b])
                    .then_with(|| nums[*a].cmp(&nums[*b]))
            });
            let mut index = vec![0; n];
            for (from, to) in sorting.into_iter().enumerate() {
                index[to] = from;
            }
            index
        };
        let mut visited = vec![false; n];
        let mut cycles = 0;
        for start in 0..n {
            if visited[start] {
                continue;
            }
            let mut current = start;
            while !visited[current] {
                visited[current] = true;
                current = sorted_index[current];
            }
            cycles += 1;
        }
        (n - cycles) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![37, 100] => 1; "example 1")]
    #[test_case(vec![22, 14, 33, 7] => 0; "example 2")]
    #[test_case(vec![18, 43, 34, 16] => 2; "example 3")]
    #[test_case(vec![268835996,65052660,415128775] => 2; "case 1")]
    #[test_case(vec![875031872,661889073,850559628] => 0; "case 2")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::min_swaps(nums)
    }
}
