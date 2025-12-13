use regex::Regex;

pub struct Solution;

impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        let pattern = Regex::new(r#"^[A-Za-z0-9_]+$"#).unwrap();
        let mut valid = vec![];
        for (i, c) in code.into_iter().enumerate() {
            if !pattern.is_match(&c)
                || !is_active[i]
                || !["electronics", "grocery", "pharmacy", "restaurant"]
                    .contains(&business_line[i].as_str())
            {
                continue;
            }
            valid.push((
                match business_line[i].as_str() {
                    "electronics" => 0,
                    "grocery" => 1,
                    "pharmacy" => 2,
                    "restaurant" => 3,
                    _ => unreachable!(),
                },
                c,
            ));
        }
        valid.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        valid.into_iter().map(|(_, a)| a).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec!["SAVE20","","PHARMA5","SAVE@20"], vec!["restaurant","grocery","pharmacy","restaurant"], vec![true, true, true, true], vec!["PHARMA5","SAVE20"]; "example 1")]
    #[test_case(vec!["1OFw","0MvB"], vec!["electronics","pharmacy"], vec![true, true], vec!["1OFw","0MvB"])]
    fn test_solution(
        code: Vec<&str>,
        business_line: Vec<&str>,
        is_active: Vec<bool>,
        expected: Vec<&str>,
    ) {
        assert_eq!(
            expected,
            Solution::validate_coupons(
                code.into_iter().map(String::from).collect(),
                business_line.into_iter().map(String::from).collect(),
                is_active
            )
        );
    }
}
