pub struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let row_count: Vec<usize> = bank
            .into_iter()
            .map(|x| x.chars().filter(|&x| x == '1').count())
            .filter(|&x| x > 0)
            .collect();
        if row_count.len() <= 1 {
            return 0;
        }
        let mut count = 0;
        for (i, row) in row_count.iter().enumerate().take(row_count.len() - 1) {
            count += row * row_count[i + 1];
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec!["011001","000000","010100","001000"] => 8; "example 1")]
    #[test_case(vec!["000","111","000"] => 0; "example 2")]
    fn test_solution(bank: Vec<&str>) -> i32 {
        Solution::number_of_beams(bank.into_iter().map(String::from).collect())
    }
}
