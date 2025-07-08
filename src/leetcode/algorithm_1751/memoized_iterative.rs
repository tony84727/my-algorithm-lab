use std::collections::{BTreeMap, HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        if events.is_empty() {
            return 0;
        }
        let mut indexed: BTreeMap<i32, Vec<Vec<i32>>> = BTreeMap::new();
        let mut last_start = 0;
        for e in events.into_iter() {
            if e[0] > last_start {
                last_start = e[0];
            }
            indexed.entry(e[0]).or_default().push(e);
        }
        let mut next_start = HashMap::new();
        let mut last = 0;
        for k in indexed.keys() {
            next_start.insert(last, k);
            last = *k;
        }
        let mut memoized = HashMap::new();
        let mut stack = vec![(0, k)];
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        while let Some((day, k)) = stack.pop() {
            let first_visit = !visited.contains(&(day, k));
            if first_visit {
                stack.push((day, k));
            }
            visited.insert((day, k));
            if let Some(_answer) = memoized.get(&(day, k)) {
                continue;
            }
            if k == 1 {
                memoized.insert(
                    (day, k),
                    if let Some(events) = indexed.get(&day) {
                        events.iter().map(|x| x[2]).max().unwrap_or_default()
                    } else {
                        0
                    },
                );
                continue;
            }
            if k == 0 {
                memoized.insert((day, k), 0);
                continue;
            }
            let mut optimal = 0;
            if let Some(today) = indexed.get(&day) {
                for e in today.iter() {
                    if first_visit {
                        stack.push((e[1] + 1, k - 1));
                        continue;
                    }
                    if let Some(tomorrow_optimal) = memoized.get(&(e[1] + 1, k - 1)) {
                        let value = e[2] + tomorrow_optimal;
                        if value > optimal {
                            optimal = value;
                        }
                    }
                }
            }
            if let Some(&&next) = next_start.get(&day) {
                if first_visit {
                    stack.push((next, k));
                } else if let Some(tomorrow_optimal) = memoized.get(&(next, k)) {
                    if *tomorrow_optimal > optimal {
                        optimal = *tomorrow_optimal
                    }
                }
            }
            if !first_visit {
                memoized.insert((day, k), optimal);
            }
        }
        *memoized.get(&(0, k)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::vecvec;

    use super::*;
    use test_case::test_case;

    #[test_case(vecvec![[1,2,4],[3,4,3],[2,3,1]], 2 => 7; "example 1")]
    #[test_case(vecvec![[1,2,4],[3,4,3],[2,3,10]], 2 => 10; "example 2")]
    #[test_case(vecvec![[459929075,951919511,721143],[148450400,967146428,484818],[293949039,390072060,496867],[664550832,931830805,731120],[330211034,558316871,338254],[81227374,667880478,548610],[147846768,194998968,612755],[351807846,540280427,341186],[54170800,836041799,244661],[330939193,441251629,47919],[54182206,101984119,150555],[554895182,796542604,423196],[58400134,787360507,799140],[241736797,338332143,899174],[373554802,993976326,671447],[941444720,951429172,939290],[106176502,513676326,558956],[240174579,745765947,661637],[239861818,464023963,967213],[106125069,235256263,351626],[280547465,625935474,645311],[106781093,240262675,926872],[589225674,785367284,501891],[748091027,871685606,212743],[581966620,828402338,818217],[118634190,931640112,904912],[237054158,906310709,713262],[99500563,210070284,227091],[431978959,728987669,745393],[149704417,939842409,697027],[432332860,460942425,734059],[221175309,737262451,474483],[331737689,553812300,434498],[915621115,984855606,545598],[24867066,914382887,392666],[68891627,513522347,115013],[918166451,960585263,489460],[408132067,469373520,139323],[608095688,610048358,363176],[469998602,661209040,2646],[325518516,752133567,132649],[138366124,851892399,416802],[344518828,848947730,991956],[49611495,268646880,486390],[65282294,319604638,28353],[97238032,408220704,37952],[345292713,378438203,570329],[201451612,336735644,150728],[247140996,896102375,947460],[467471259,565647118,441446]], 27 => 5756021; "case 1")]
    #[test_case(vec![vec![74,91,40]], 1 => 40; "case 2")]
    fn test_solution(events: Vec<Vec<i32>>, k: i32) -> i32 {
        Solution::max_value(events, k)
    }
}
