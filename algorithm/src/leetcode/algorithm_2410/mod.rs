pub struct Solution;

impl Solution {
    pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
        players.sort_unstable();
        trainers.sort_unstable();
        let mut trainers = trainers.into_iter();
        let mut count = 0;
        for p in players.into_iter() {
            for t in trainers.by_ref() {
                if t >= p {
                    count += 1;
                    break;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![4,7,9], vec![8,2,5,8] => 2; "example 1")]
    #[test_case(vec![1,1,1], vec![10] =>1; "example 2")]
    fn test_solution(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        Solution::match_players_and_trainers(players, trainers)
    }
}
