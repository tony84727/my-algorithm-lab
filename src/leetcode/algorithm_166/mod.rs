pub struct Solution;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let output = (numerator as f64 / denominator as f64).to_string();
        let parts: Vec<&str> = output.split(".").collect();
        if parts.len() == 1 {
            return output;
        }
        for prefix in 0..parts[1].len() {
            for j in (1..(parts[1].len() - prefix) / 2).rev() {
                let (first_half, second_half) = &parts[1][prefix..].split_at(j);
                if first_half[..j] == second_half[..j] {
                    return format!("{}.{}({})", parts[0], &parts[1][..prefix], &first_half[..j]);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1, 2 => "0.5"; "example 1")]
    #[test_case(2, 1 => "2"; "example 2")]
    #[test_case(4, 333 => "0.(012)"; "example 3")]
    #[test_case(1, 17 => "0.(0588235294117647)"; "case 1")]
    fn test_solution(numerator: i32, denominator: i32) -> String {
        Solution::fraction_to_decimal(numerator, denominator)
    }
}
