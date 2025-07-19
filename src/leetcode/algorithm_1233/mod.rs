pub struct Solution;

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();

        let mut run = true;
        while run {
            let Some(prefix) = folder.first() else {
                return Vec::new();
            };
            let mut remaining = vec![prefix.clone()];
            let mut prefix = prefix.clone() + "/";
            run = false;
            for p in folder.iter().skip(1) {
                if p.starts_with(&prefix) {
                    run = true;
                    continue;
                }
                remaining.push(p.clone());
                prefix = p.clone() + "/";
            }
            folder = remaining;
        }
        folder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&["/a","/a/b","/c/d","/c/d/e","/c/f"], &["/a","/c/d","/c/f"]; "example 1")]
    #[test_case(&["/a","/a/b/c","/a/b/d"], &["/a"]; "example 2")]
    #[test_case(&["/a/b/c","/a/b/ca","/a/b/d"], &["/a/b/c","/a/b/ca","/a/b/d"]; "example 3")]
    fn test_solution(folder: &[&str], expected: &[&str]) {
        let output =
            Solution::remove_subfolders(folder.iter().cloned().map(String::from).collect());
        assert_eq!(
            expected
                .iter()
                .cloned()
                .map(String::from)
                .collect::<Vec<String>>(),
            output,
        );
    }
}
