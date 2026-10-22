pub struct Solution;

impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut lowest_obstacles = vec![];
        obstacles
            .into_iter()
            .map(|obstacle| Self::binary_search_insert(&mut lowest_obstacles, obstacle) as i32 + 1)
            .collect()
    }

    fn binary_search_insert(lowests: &mut Vec<i32>, obstacle: i32) -> usize {
        if lowests.is_empty() {
            lowests.push(obstacle);
            return 0;
        }
        let mut right = lowests.len();
        let mut left = 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if lowests[mid - 1] <= obstacle {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        let index = left - 1;
        if index >= lowests.len() {
            lowests.push(obstacle)
        } else {
            lowests[index] = obstacle;
        }
        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,2] => vec![1,2,3,3]; "example 1")]
    #[test_case(vec![2,2,1] => vec![1,2,1]; "example 2")]
    #[test_case(vec![3,1,5,6,4,2] => vec![1,1,2,3,2,2]; "example 3")]
    #[test_case(vec![5,3,4,4,4,2,1,1,4,1] => vec![1,1,2,3,4,1,1,2,5,3]; "case 1")]
    fn test_solution(obstacles: Vec<i32>) -> Vec<i32> {
        Solution::longest_obstacle_course_at_each_position(obstacles)
    }
}
