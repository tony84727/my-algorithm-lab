use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn dfs(
        fuel: &mut i64,
        node: i32,
        parent: i32,
        transition: &HashMap<i32, Vec<i32>>,
        seats: i32,
    ) -> i32 {
        let count = transition
            .get(&node)
            .unwrap_or(&vec![])
            .iter()
            .filter(|child| **child != parent)
            .map(|child| Self::dfs(fuel, *child, node, transition, seats))
            .sum::<i32>()
            + 1;
        if node != 0 {
            *fuel += (count as f32 / seats as f32).ceil() as i64;
        }
        count
    }
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let mut transitions = HashMap::<i32, Vec<i32>>::new();
        for edge in roads.into_iter() {
            transitions.entry(edge[0]).or_default().push(edge[1]);
            transitions.entry(edge[1]).or_default().push(edge[0]);
        }
        let mut fuel = 0;
        Self::dfs(&mut fuel, 0, 0, &transitions, seats);
        fuel
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![(0,1),(0,2), (0,3)], 5 => 3; "example1")]
    #[test_case(vec![(3,1), (3,2), (1,0), (0,4), (0,5), (4,6)], 2 => 7; "example 2")]
    fn test_solution(roads: Vec<(i32, i32)>, seats: i32) -> i64 {
        Solution::minimum_fuel_cost(roads.into_iter().map(|(a, b)| vec![a, b]).collect(), seats)
    }
}
