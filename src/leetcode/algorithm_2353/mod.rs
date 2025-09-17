use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Eq, PartialEq)]
struct Entry {
    food: String,
    version: usize,
    rating: i32,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let rating_cmp = self.rating.cmp(&other.rating);
        if rating_cmp != Ordering::Equal {
            return rating_cmp;
        }
        other.food.cmp(&self.food)
    }
}

pub struct FoodRatings {
    cuisines: HashMap<String, BinaryHeap<Entry>>,
    food_cuisine: HashMap<String, String>,
    versions: HashMap<String, usize>,
}

impl FoodRatings {
    pub fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut cuisine_heaps = HashMap::new();
        let mut versions = HashMap::new();
        let mut food_cuisine = HashMap::new();

        for ((f, c), r) in foods
            .into_iter()
            .zip(cuisines.into_iter())
            .zip(ratings.into_iter())
        {
            versions.insert(f.clone(), 0);
            food_cuisine.insert(f.clone(), c.clone());
            cuisine_heaps
                .entry(c)
                .or_insert_with(BinaryHeap::new)
                .push(Entry {
                    food: f,
                    version: 0,
                    rating: r,
                });
        }

        Self {
            cuisines: cuisine_heaps,
            food_cuisine,
            versions,
        }
    }

    pub fn change_rating(&mut self, food: String, new_rating: i32) {
        let version = self.versions.entry(food.clone()).or_default();
        *version += 1;
        let cuisine = self.food_cuisine.get(&food).unwrap();
        self.cuisines.get_mut(cuisine).unwrap().push(Entry {
            food,
            version: *version,
            rating: new_rating,
        })
    }

    pub fn highest_rated(&mut self, cuisine: String) -> String {
        let heap = self.cuisines.get_mut(&cuisine).unwrap();
        while !heap.is_empty() {
            let largest = heap.peek().unwrap();
            let version = self.versions.get(&largest.food).unwrap();
            if version == &largest.version {
                return largest.food.clone();
            }
            heap.pop();
        }
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_example_1() {
        let mut ratings = FoodRatings::new(
            vec!["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"]
                .into_iter()
                .map(String::from)
                .collect(),
            vec![
                "korean", "japanese", "japanese", "greek", "japanese", "korean",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            vec![9, 12, 8, 15, 14, 7],
        );
        assert_eq!("kimchi", ratings.highest_rated(String::from("korean")));
        assert_eq!("ramen", ratings.highest_rated(String::from("japanese")));
        ratings.change_rating(String::from("sushi"), 16);
        assert_eq!("sushi", ratings.highest_rated(String::from("japanese")));
        ratings.change_rating(String::from("ramen"), 16);
        assert_eq!("ramen", ratings.highest_rated(String::from("japanese")));
    }
}
