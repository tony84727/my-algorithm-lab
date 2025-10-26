pub struct Bank {
    balance: Vec<i64>,
}

impl Bank {
    pub fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }

    pub fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let n = self.balance.len();
        let account1 = account1 as usize;
        let account2 = account2 as usize;
        if !(1..=n).contains(&account1) || !(1..=n).contains(&account2) {
            return false;
        }
        if self.balance[account1 - 1] < money {
            return false;
        }
        self.balance[account1 - 1] -= money;
        self.balance[account2 - 1] += money;
        true
    }

    pub fn deposit(&mut self, account: i32, money: i64) -> bool {
        let n = self.balance.len();
        let account = account as usize;
        if !(1..=n).contains(&account) {
            return false;
        }
        self.balance[account - 1] += money;
        true
    }

    pub fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let n = self.balance.len();
        let account = account as usize;
        if !(1..=n).contains(&account) {
            return false;
        }
        if self.balance[account - 1] < money {
            return false;
        }
        self.balance[account - 1] -= money;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solution_example_1() {
        let mut bank = Bank::new(vec![10, 200, 20, 50, 30]);
        assert!(bank.withdraw(3, 10));
        assert!(bank.transfer(5, 1, 20));
        assert!(bank.deposit(5, 20));
        assert!(!bank.transfer(3, 4, 15));
        assert!(!bank.withdraw(10, 50));
    }
}
