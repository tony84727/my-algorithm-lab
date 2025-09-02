pub struct Solution;

impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        let mut anchor_x = i32::MAX;
        let mut anchor_y = 0;
        for p in points.iter() {
            anchor_x = anchor_x.min(p[0]);
            anchor_y = anchor_y.max(p[1]);
        }
        points.sort_unstable_by_key(|a| (a[0] - anchor_x).pow(2) + (a[1] - anchor_y).pow(2));
        let mut count = 0;
        let mut x = anchor_x;
        let mut y = anchor_y;
        let first_point = points.first().unwrap();
        let anchor_exist = first_point[0] == x && first_point[1] == y;
        for p in points.iter().skip(1) {
            if p[0] < x || p[1] > y {
                continue;
            }
            x = p[0];
            y = p[1];
            count += 1;
        }
        if anchor_exist {
            count
        } else {
            count - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[1,1],[2,2],[3,3]] => 0; "example 1")]
    #[test_case(vecvec![[6,2],[4,4],[2,6]] => 2; "example 2")]
    #[test_case(vecvec![[3,1],[1,3],[1,1]] => 2; "example 3")]
    #[test_case(vecvec![[0,1],[1,3],[6,1]] => 2; "case 1")]
    fn test_solution(points: Vec<Vec<i32>>) -> i32 {
        Solution::number_of_pairs(points)
    }
}
