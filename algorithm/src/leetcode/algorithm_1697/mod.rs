pub mod brute;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(3, &[(0,1,2), (1,2,4), (2,0,8), (1,0,16)], &[(0,1,2), (0,2,5)] => vec![false, true]; "example 1")]
    fn test_brute(n: i32, lists: &[(i32, i32, i32)], queries: &[(i32, i32, i32)]) -> Vec<bool> {
        brute::Solution::distance_limited_paths_exist(
            n,
            lists.iter().map(|(a, b, c)| vec![*a, *b, *c]).collect(),
            queries.iter().map(|(a, b, c)| vec![*a, *b, *c]).collect(),
        )
    }
}
