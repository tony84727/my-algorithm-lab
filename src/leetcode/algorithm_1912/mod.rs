use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

#[derive(Eq, PartialEq)]
struct Slot {
    movie: i32,
    shop: i32,
    price: i32,
    rented: bool,
}

impl PartialOrd for Slot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Slot {
    fn cmp(&self, other: &Self) -> Ordering {
        let price = self.price.cmp(&other.price);
        if price != Ordering::Equal {
            return price;
        }
        let shop = self.shop.cmp(&other.shop);
        if shop != Ordering::Equal {
            return shop;
        }
        self.movie.cmp(&other.movie)
    }
}

#[derive(Default)]
pub struct MovieRentingSystem {
    movies: HashMap<i32, HashMap<i32, Rc<RefCell<Slot>>>>,
    search_rank: HashMap<i32, Vec<Rc<RefCell<Slot>>>>,
    report_rank: BinaryHeap<Reverse<Rc<RefCell<Slot>>>>,
}

impl MovieRentingSystem {
    pub fn new(_n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut instance = Self::default();
        for e in entries.into_iter() {
            let shop = e[0];
            let movie = e[1];
            let price = e[2];
            let slot = Rc::new(RefCell::new(Slot {
                movie,
                shop,
                price,
                rented: false,
            }));
            instance
                .movies
                .entry(movie)
                .or_default()
                .insert(shop, slot.clone());
            instance.search_rank.entry(movie).or_default().push(slot);
        }
        for slots in instance.search_rank.values_mut() {
            slots.sort_unstable();
        }
        instance
    }

    pub fn search(&self, movie: i32) -> Vec<i32> {
        self.search_rank
            .get(&movie)
            .map(|x| {
                x.iter()
                    .filter(|s| !s.borrow().rented)
                    .take(5)
                    .map(|s| s.borrow().shop)
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn rent(&mut self, shop: i32, movie: i32) {
        let slot = self.movies.get_mut(&movie).unwrap().get_mut(&shop).unwrap();
        slot.borrow_mut().rented = true;
        self.report_rank.push(Reverse(slot.clone()));
    }

    pub fn drop(&mut self, shop: i32, movie: i32) {
        let slot = self.movies.get_mut(&movie).unwrap().get_mut(&shop).unwrap();
        slot.borrow_mut().rented = false;
    }

    pub fn report(&mut self) -> Vec<Vec<i32>> {
        let mut collected = Vec::new();
        let mut to_push = vec![];
        while !self.report_rank.is_empty() && collected.len() < 5 {
            let Reverse(current) = self.report_rank.pop().unwrap();
            if !current.borrow().rented {
                continue;
            }
            {
                let current_ref = current.borrow();
                let shop = current_ref.shop;
                let movie = current_ref.movie;
                if let Some(last) = to_push.last() {
                    if last == &current {
                        continue;
                    }
                }
                collected.push(vec![shop, movie]);
            }
            to_push.push(current);
        }
        for p in to_push.into_iter() {
            self.report_rank.push(Reverse(p));
        }
        collected
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;

    #[test]
    fn test_solution_example_1() {
        let mut system = MovieRentingSystem::new(
            3,
            vecvec![
                [0, 1, 5],
                [0, 2, 6],
                [0, 3, 7],
                [1, 1, 4],
                [1, 2, 7],
                [2, 1, 5]
            ],
        );
        assert_eq!(vec![1, 0, 2], system.search(1));
        system.rent(0, 1);
        system.rent(1, 2);
        assert_eq!(vecvec![[0, 1], [1, 2]], system.report());
        system.drop(1, 2);
        assert_eq!(vec![0, 1], system.search(2));
    }
}

