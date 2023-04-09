pub struct Solution;

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let mut neighbors = vec![vec![]; colors.len()];
        let mut in_degress = vec![0; colors.len()];
        for e in edges.into_iter() {
            let from = e[0] as usize;
            let to = e[1];
            neighbors[from].push(to);
            in_degress[to as usize] += 1;
        }
        let colors = colors.chars().collect::<Vec<char>>();
        let mut color_counts = vec![vec![0; 26]; colors.len()];
        let mut todo = vec![];
        for (i, n) in in_degress.iter().enumerate() {
            if *n == 0 {
                todo.push(i);
            }
        }
        let mut max = 0;
        let mut visited = 0;

        while let Some(current) = todo.pop() {
            let color = colors[current];
            let color_index = (color as u8 - 'a' as u8) as usize;
            color_counts[current][color_index] += 1;
            max = max.max(color_counts[current][color_index]);
            visited += 1;
            for neighbor in neighbors[current].iter() {
                let neighbor = *neighbor as usize;
                for color in 0..26 {
                    color_counts[neighbor][color] =
                        color_counts[neighbor][color].max(color_counts[current][color]);
                }
                in_degress[neighbor] -= 1;
                if in_degress[neighbor] == 0 {
                    todo.push(neighbor);
                }
            }
        }
        if visited < colors.len() {
            -1
        } else {
            max
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abaca", vec![vec![0,1], vec![0,2], vec![2,3], vec![3,4]] => 3; "example 1")]
    fn test_solution(colors: &str, edges: Vec<Vec<i32>>) -> i32 {
        Solution::largest_path_value(colors.to_owned(), edges)
    }
}
