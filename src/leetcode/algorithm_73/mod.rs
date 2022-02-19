pub mod skip;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_macro::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[0,0,0,5], [4,3,1,4], [0,1,1,4], [1,2,1,3], [0,0,1,1]],vecvec![[0,0,0,0], [0,0,0,4], [0,0,0,0], [0,0,0,3], [0,0,0,0]]; "case 1")]
    fn test_skip_solution(mut input: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
        skip::Solution::set_zeroes(&mut input);
        assert_eq!(expected, input);
    }
}
