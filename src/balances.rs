use std::{collections::BTreeMap, fmt::Debug};

use num::{CheckedAdd, CheckedSub, Zero};

pub trait Config {
    type AccountId: Ord + Clone;
    type Balance: Zero + CheckedAdd + CheckedSub + Copy + Debug;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Pallet {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, account: &T::AccountId, amount: T::Balance) {
        self.balances.insert(account.clone(), amount);
    }

    pub fn balance(&mut self, account: &T::AccountId) -> T::Balance {
        *self.balances.get(account).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(
        &mut self,
        from: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let from_amount = self.balance(&from);
        let to_amount = self.balance(&to);

        println!("from amount {:#?}", from_amount);

        let new_from_amount = (from_amount)
            .checked_sub(&amount)
            .ok_or("Insufficient fund")?;

        let new_to_amount = (to_amount)
            .checked_add(&amount)
            .ok_or("Overflow when adding to balance")?;

        self.set_balance(&from, new_from_amount);
        self.set_balance(&to, new_to_amount);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestConfig {}

    impl Config for TestConfig {
        type AccountId = String;
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut balances: Pallet<TestConfig> = Pallet::new();
        assert_eq!(balances.balance(&"A".to_string()), 0);
        balances.set_balance(&"A".to_string(), 100);

        assert_eq!(balances.balance(&"A".to_string()), 100);
    }

    #[test]
    fn transfer_balance() {
        let a = "A".to_string();
        let b = "B".to_string();
        let mut balances: Pallet<TestConfig> = Pallet::new();
        balances.set_balance(&a, 100);
        let _ = balances.transfer(a.clone(), b.clone(), 50);

        assert_eq!(balances.balance(&a), 50);
        assert_eq!(balances.balance(&b), 50);
    }

    #[test]
    fn transfer_insufficient_balance() {
        let a = "A".to_string();
        let b = "B".to_string();
        let mut balances: Pallet<TestConfig> = Pallet::new();
        let result = balances.transfer(a.clone(), b.clone(), 200);

        assert_eq!(result, Err("Insufficient fund"));
        assert_eq!(balances.balance(&a), 0);
        assert_eq!(balances.balance(&b), 0);
    }

    #[test]
    fn transfer_overflow_balance() {
        let a = "A".to_string();
        let b = "B".to_string();
        let mut balances: Pallet<TestConfig> = Pallet::new();
        balances.set_balance(&a, u128::MAX);
        balances.set_balance(&b, 100);
        let result = balances.transfer(b.clone(), a.clone(), 50);

        assert_eq!(result, Err("Overflow when adding to balance"));
        assert_eq!(balances.balance(&a), u128::MAX);
        assert_eq!(balances.balance(&b), 100);
    }
}
