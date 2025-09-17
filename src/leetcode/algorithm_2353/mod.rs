use std::collections::HashMap;

pub struct FoodRatings {
    cuisine_highest: HashMap<String, String>,
    cuisine_highest_rating: HashMap<String, i32>,
    food_cuisine: HashMap<String, String>,
}

impl FoodRatings {
    pub fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut cuisine_highest = HashMap::new();
        let mut cuisine_highest_rating = HashMap::new();
        let mut food_cuisine = HashMap::new();

        for ((f, c), r) in foods
            .into_iter()
            .zip(cuisines.into_iter())
            .zip(ratings.into_iter())
        {
            food_cuisine.insert(f.clone(), c.clone());
            let highest_rating = cuisine_highest_rating.entry(c.clone()).or_default();
            if *highest_rating < r {
                *highest_rating = r;
                cuisine_highest.insert(c, f);
            }
        }

        Self {
            cuisine_highest,
            cuisine_highest_rating,
            food_cuisine,
        }
    }

    pub fn change_rating(&mut self, food: String, new_rating: i32) {
        let cuisine = self.food_cuisine.get(&food).unwrap();
        let highest_rating = self
            .cuisine_highest_rating
            .entry(cuisine.clone())
            .or_default();
        if new_rating > *highest_rating {
            *highest_rating = new_rating;
            self.cuisine_highest.insert(cuisine.clone(), food);
        }
    }

    pub fn highest_rated(&self, cuisine: String) -> String {
        self.cuisine_highest.get(&cuisine).unwrap().clone()
    }
}
