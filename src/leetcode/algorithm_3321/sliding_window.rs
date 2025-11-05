use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Eq, PartialEq)]
struct Entry(usize, i32);

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Soution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let n = nums.len();
        let k = k as usize;
        let x = x as usize;
        let mut frequency: HashMap<i32, usize> = HashMap::new();
        let mut current = BTreeSet::new();
        let mut answer = Vec:new();
        let mut sum = 0;
        for &num in nums.iter().take(k) {
            let count = frequency.entry(num).or_default();
            *count += 1;
            sum += num;
            if current.len() < x {
                current.insert(Entry(*count, num));
                continue;
            }
            let Entry(min_count, min_i) = current.range(..).next().unwrap();
            if *count > *min_count || (*count == *min_count && num > min_i) {
                sum -= min_i * (*min_count) as i32;
                sum += num*(*count) as i32;
            }
        }
        answer.push(sum);
        for (i, &pop) in nums.iter().enumerate().skip(1).take(n-k) {
        }
        answer
    }
}
