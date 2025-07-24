pub struct Solution;

impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let total = nums.iter().fold(0, |a, b| a ^ b);
        let mut adjacent_list: Vec<Vec<usize>> = vec![vec![]; nums.len()];
        for e in edges.iter() {
            adjacent_list[e[0] as usize].push(e[1] as usize);
            adjacent_list[e[1] as usize].push(e[0] as usize);
        }
        let mut min = i32::MAX;
        Self::dfs(&nums, &adjacent_list, total, 0, None, &mut min);
        min
    }

    fn dfs(
        nums: &[i32],
        adjacent_list: &Vec<Vec<usize>>,
        total: i32,
        node: usize,
        ignore: Option<usize>,
        min: &mut i32,
    ) -> i32 {
        let mut sum = nums[node];
        for &c in adjacent_list[node].iter() {
            if Some(c) == ignore {
                continue;
            }
            sum ^= Self::dfs(nums, adjacent_list, total, c, Some(node), min);
        }
        for &c in adjacent_list[node].iter() {
            if Some(c) == ignore {
                Self::dfs2(nums, adjacent_list, total, node, c, node, min, sum);
            }
        }
        sum
    }

    #[allow(clippy::too_many_arguments)]
    fn dfs2(
        nums: &[i32],
        adjacent_list: &Vec<Vec<usize>>,
        total: i32,
        first_root: usize,
        node: usize,
        ignore: usize,
        min: &mut i32,
        first_sum: i32,
    ) -> i32 {
        let mut sum = nums[node];
        for &c in adjacent_list[node].iter() {
            if c == ignore {
                continue;
            }
            sum ^= Self::dfs2(
                nums,
                adjacent_list,
                total,
                first_root,
                c,
                node,
                min,
                first_sum,
            );
        }
        if first_root == ignore {
            return sum;
        }
        let mut scores = [first_sum, sum, total ^ first_sum ^ sum];
        scores.sort();
        *min = (*min).min(scores[2] - scores[0]);
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vec![1,5,5,4,11], vecvec![[0,1],[1,2],[1,3],[3,4]] => 9; "example 1")]
    #[test_case(vec![5,5,2,4,4,2], vecvec![[0,1],[1,2],[5,2],[4,3],[1,3]] => 0; "example 2")]
    #[test_case(vec![29,29,23,32,17], vecvec![[3,1],[2,3],[4,1],[0,4]] => 15; "case 1")]
    #[test_case(vec![18,4,5,27,22,18], vecvec![[5,4],[2,4],[2,0],[4,1],[3,2]] => 6; "case 2")]
    fn test_solution(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        Solution::minimum_score(nums, edges)
    }
}
