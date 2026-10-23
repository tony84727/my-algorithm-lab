pub struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn find_paths(graph: &[Vec<i32>], from: i32, to: i32) -> Vec<Vec<i32>> {
            graph[from as usize]
                .iter()
                .flat_map(|&next| {
                    if next == to {
                        vec![vec![to, from]]
                    } else {
                        find_paths(graph, next, to)
                            .into_iter()
                            .filter(|paths| !paths.is_empty())
                            .map(|mut paths| {
                                paths.push(from);
                                paths
                            })
                            .collect()
                    }
                })
                .collect()
        }
        find_paths(&graph, 0, (graph.len() - 1) as i32)
            .into_iter()
            .map(|mut paths| {
                paths.reverse();
                paths
            })
            .collect()
    }
}
