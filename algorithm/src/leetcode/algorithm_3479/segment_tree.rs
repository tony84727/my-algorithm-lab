pub struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut segement_max = vec![0; 4 * baskets.len()];
        Self::build_segement(&baskets, &mut segement_max, 1, 0, baskets.len() - 1);
        let mut count = 0;
        for f in fruits.into_iter() {
            if !Self::first_and_update(&mut segement_max, f, 1, 0, baskets.len() - 1) {
                count += 1;
            }
        }
        count
    }

    fn build_segement(baskets: &[i32], nodes: &mut [i32], node: usize, left: usize, right: usize) {
        if right == left {
            nodes[node] = baskets[right];
            return;
        }
        let mid = (right + left) / 2;
        Self::build_segement(baskets, nodes, node * 2, left, mid);
        Self::build_segement(baskets, nodes, node * 2 + 1, mid + 1, right);
        nodes[node] = nodes[node * 2].max(nodes[node * 2 + 1]);
    }

    fn first_and_update(
        nodes: &mut [i32],
        count: i32,
        node: usize,
        left: usize,
        right: usize,
    ) -> bool {
        if nodes[node] < count {
            return false;
        }
        if right == left {
            nodes[node] = -1;
            return true;
        }
        let mid = (right + left) / 2;
        let result = Self::first_and_update(nodes, count, 2 * node, left, mid)
            || Self::first_and_update(nodes, count, 2 * node + 1, mid + 1, right);
        nodes[node] = nodes[2 * node].max(nodes[2 * node + 1]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![4,2,5], vec![3,5,4] => 1; "example 1")]
    #[test_case(vec![3,6,1], vec![6,4,7] => 0; "example 2")]
    fn test_solution(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        Solution::num_of_unplaced_fruits(fruits, baskets)
    }
}
