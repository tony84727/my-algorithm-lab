pub struct Solution;

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut first_interval = 0;
        let mut second_internval = 0;
        let mut intersections = vec![];
        while first_interval < first_list.len() && second_internval < second_list.len() {
            let first = &first_list[first_interval];
            let second = &second_list[second_internval];
            let start = if first[0] > second[0] {
                first[0]
            } else {
                second[0]
            };
            let end = if first[1] > second[1] {
                second[1]
            } else {
                first[1]
            };
            if start <= end {
                intersections.push(vec![start, end]);
            }
            if first[1] <= end {
                first_interval += 1;
            }
            if second[1] <= end {
                second_internval += 1;
            }
        }
        intersections
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use test_case::test_case;

    #[derive(Deserialize)]
    struct TestCase {
        first: Vec<Vec<i32>>,
        second: Vec<Vec<i32>>,
        output: Vec<Vec<i32>>,
    }

    #[test_case("src/leetcode/algorithm_986/example_1.ron"; "example 1")]
    fn test_solution(testcase_path: &str) {
        let TestCase {
            first,
            second,
            output,
        } = {
            let mut f = std::fs::File::open(testcase_path).unwrap();
            ron::de::from_reader(&mut f).unwrap()
        };
        assert_eq!(output, Solution::interval_intersection(first, second))
    }
}
