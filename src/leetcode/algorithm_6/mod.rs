pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let mut rows = vec![String::new(); num_rows as usize];
        let mut row = 0_i32;
        let mut direction = 1;
        for c in s.chars() {
            rows[row as usize].push(c);
            row += direction;
            if row >= num_rows - 1 || row <= 0 {
                direction *= -1;
            }
        }
        rows.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("PAYPALISHIRING", 3 => "PAHNAPLSIIGYIR".to_string(); "example 1")]
    #[test_case("PAYPALISHIRING", 4 => "PINALSIGYAHRPI"; "example 2")]
    #[test_case("AB", 1 => "AB"; "case 1")]
    fn test_solution(s: &str, num_rows: i32) -> String {
        Solution::convert(s.to_string(), num_rows)
    }
}
