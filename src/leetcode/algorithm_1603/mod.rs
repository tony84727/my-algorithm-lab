pub struct ParkingSystem {
    counts: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            counts: vec![big, medium, small],
        }
    }

    pub fn add_car(&mut self, car_type: i32) -> bool {
        if self.counts[car_type as usize - 1] <= 0 {
            return false;
        }
        self.counts[car_type as usize - 1] -= 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let mut system = ParkingSystem::new(1, 1, 0);
        assert!(system.add_car(1));
        assert!(system.add_car(2));
        assert!(!system.add_car(3));
        assert!(!system.add_car(1));
        assert!(!system.add_car(1));
    }
}
