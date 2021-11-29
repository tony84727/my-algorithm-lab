use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut cache = HashMap::new();
        fn find_paths(
            cache: &mut HashMap<i32, Vec<Vec<i32>>>,
            graph: &[Vec<i32>],
            from: i32,
            to: i32,
        ) -> Vec<Vec<i32>> {
            if let Some(answer) = cache.get(&from) {
                return answer.clone();
            }
            let answer: Vec<Vec<i32>> = graph[from as usize]
                .iter()
                .map(|&next| {
                    if next == to {
                        vec![vec![to, from]]
                    } else {
                        find_paths(cache, graph, next, to)
                            .into_iter()
                            .filter(|paths| !paths.is_empty())
                            .map(|mut paths| {
                                paths.push(from);
                                paths
                            })
                            .collect()
                    }
                })
                .flatten()
                .collect();
            cache.insert(from, answer.clone());
            answer
        }
        find_paths(&mut cache, &graph, 0, (graph.len() - 1) as i32)
            .into_iter()
            .map(|mut paths| {
                paths.reverse();
                paths
            })
            .collect()
    }
}
