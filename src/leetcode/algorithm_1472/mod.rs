pub struct BrowserHistory {
    history: Vec<String>,
    current: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {
    pub fn new(homepage: String) -> Self {
        Self {
            history: vec![homepage],
            current: 0,
        }
    }

    pub fn visit(&mut self, url: String) {
        self.history.resize(self.current + 1, String::new());
        self.history.push(url);
        self.current += 1;
    }

    pub fn back(&mut self, steps: i32) -> String {
        if steps as usize > self.current {
            self.current = 0;
        } else {
            self.current -= steps as usize;
        }
        self.history[self.current].clone()
    }

    pub fn forward(&mut self, steps: i32) -> String {
        self.current = (self.history.len() - 1).min(self.current + steps as usize);
        self.history[self.current].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::BrowserHistory;

    #[test]
    fn test_solution_example1() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        history.visit("google.com".to_string());
        history.visit("facebook.com".to_string());
        history.visit("youtube.com".to_string());
        assert_eq!("facebook.com", history.back(1));
        assert_eq!("google.com", history.back(1));
        assert_eq!("facebook.com", history.forward(1));
        history.visit("linkedin.com".to_string());
        assert_eq!("linkedin.com", history.forward(2));
        assert_eq!("google.com", history.back(2));
        assert_eq!("leetcode.com", history.back(7));
    }
}
