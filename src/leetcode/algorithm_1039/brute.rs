pub struct Solution;

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        Self::min_weight_of_triangles(&values)
    }

    fn split_vertex(vertexs: &[i32], a: usize, b: usize) -> (Vec<i32>, Vec<i32>) {
        let (a, b) = (a.min(b), a.max(b));
        let mut a_vertex = Vec::new();
        let mut b_vertex = Vec::new();
        for (i, v) in vertexs.iter().enumerate() {
            if i == a || i == b {
                a_vertex.push(*v);
                b_vertex.push(*v);
                continue;
            }
            let mut target = &mut a_vertex;
            if (a..b).contains(&i) {
                target = &mut b_vertex;
            }
            target.push(*v);
        }
        (a_vertex, b_vertex)
    }

    fn min_weight_of_triangles(vertexs: &[i32]) -> i32 {
        if vertexs.len() == 3 {
            return vertexs.iter().product();
        }
        let n = vertexs.len();
        let k = n.div_ceil(2);
        let mut min = i32::MAX;
        for a in 0..k {
            for b in 0..n {
                if b == a || b == (a + 1) % n || b == (a + n - 1) % n {
                    continue;
                }
                let (a_vertex, b_vertex) = Self::split_vertex(vertexs, a, b);
                min = min.min(
                    Self::min_weight_of_triangles(&a_vertex)
                        + Self::min_weight_of_triangles(&b_vertex),
                )
            }
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_split_vertex() {
        let vertex = vec![1, 3, 4, 2, 6];
        assert_eq!(
            (vec![1, 4, 2, 6], vec![1, 3, 4]),
            Solution::split_vertex(&vertex, 0, 2)
        );
    }

    #[test_case(vec![1,2,3] => 6; "example 1")]
    #[test_case(vec![3,7,4,5] => 144; "example 2")]
    #[test_case(vec![1,3,1,4,1,5] => 13; "example 3")]
    #[test_case(vec![1,2,3,4] => 18; "case 1")]
    #[test_case(vec![2,1,4,4] => 24; "case 2")]
    fn test_solution(vertex: Vec<i32>) -> i32 {
        Solution::min_score_triangulation(vertex)
    }
}
