pub mod naive;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0b00000010100101000001111010011100_u32, 0b00111001011110000010100101000000_u32; "example 1")]
    fn test_naive_solution(input: u32, expected: u32) {
        let actual = naive::Solution::reverse_bits(input);
        assert_eq!(
            expected, actual,
            "\n{:<8}: {:032b}\n{:<8}: {:032b}",
            "expected", expected, "got", actual
        );
    }
}
