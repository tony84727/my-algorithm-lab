pub struct Solution;

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let middle = directions.trim_start_matches("L").trim_end_matches("R");
        let n = middle.len();
        let mut s = 0;
        for c in middle.chars() {
            if c == 'S' {
                s += 1;
            }
        }
        (n - s) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("RLRSLL" => 5; "example 1")]
    #[test_case("LLRR" => 0; "example 2")]
    #[test_case("RRLL" => 4; "case 1")]
    fn test_solution(directions: &str) -> i32 {
        Solution::count_collisions(String::from(directions))
    }
}
