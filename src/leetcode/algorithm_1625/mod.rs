use std::cmp::Ordering;

#[derive(PartialEq, Eq, Clone)]
struct Combination(Vec<i32>);

impl Combination {
    fn rotate(&mut self, step: usize) {
        self.0.rotate_right(step);
    }

    fn add(&self, offset: i32) -> Self {
        if offset == 0 {
            return self.clone();
        }
        let mut digits = self.0.clone();
        for (_, d) in digits
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| !i.is_multiple_of(2))
        {
            *d += offset;
            *d %= 10;
        }
        Self(digits)
    }

    fn add_even(&self, offset: i32) -> Self {
        if offset == 0 {
            return self.clone();
        }
        let mut digits = self.0.clone();
        for (_, d) in digits
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| i.is_multiple_of(2))
        {
            *d += offset;
            *d %= 10;
        }
        Self(digits)
    }
}

impl PartialOrd for Combination {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Combination {
    fn cmp(&self, other: &Self) -> Ordering {
        for (i, a) in self.0.iter().enumerate() {
            let cmp = a.cmp(&other.0[i]);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }
        Ordering::Equal
    }
}

pub struct Solution;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let n = s.len();
        let s: Vec<i32> = s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        let init = Combination(s);
        let mut min = init.clone();
        let gcd = Self::gcd(10, a);
        let offsets = (0..10).step_by(gcd as usize);
        for offset in offsets.clone() {
            for even_offset in offsets.clone() {
                let mut current = if b % 2 == 0 {
                    init.add(offset)
                } else {
                    init.add(offset).add_even(even_offset)
                };
                let rotate_gcd = Self::gcd(2, b);
                for _ in (0..n).step_by(rotate_gcd as usize) {
                    current.rotate(b as usize);
                    min = min.min(current.clone());
                }
                if b % 2 == 0 {
                    break;
                }
            }
        }
        String::from_iter(min.0.iter().map(|x| x.to_string()))
    }

    fn gcd(a: i32, b: i32) -> i32 {
        let (mut a, mut b) = (a, b);
        while a != 0 && b != 0 {
            if a < b {
                (a, b) = (b, a);
            }
            a %= b;
        }
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("5525", 9, 2 => "2050"; "example 1")]
    #[test_case("74", 5, 1 => "24"; "example 2")]
    #[test_case("0011", 4, 2 => "0011"; "example 3")]
    #[test_case("43987654", 7, 3 => "00553311"; "case 1")]
    #[test_case("31", 4, 1 => "11"; "case 2")]
    #[test_case("48721802531280019397498945", 6, 12 => "00511088099195478743467016"; "case 3")]
    fn test_solution(s: &str, a: i32, b: i32) -> String {
        Solution::find_lex_smallest_string(String::from(s), a, b)
    }
}
