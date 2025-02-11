use std::{collections::BTreeMap, fmt::Debug};

use num::{CheckedAdd, CheckedSub, Zero};

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
    AccountId: Ord + Clone,
    Balance: Zero + CheckedAdd + CheckedSub + Copy + Debug,
{
    pub fn new() -> Self {
        Pallet {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, account: &AccountId, amount: Balance) {
        self.balances.insert(account.clone(), amount);
    }

    pub fn balance(&mut self, account: &AccountId) -> Balance {
        *self.balances.get(account).unwrap_or(&Balance::zero())
    }

    pub fn transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), &'static str> {
        let from_amount = self.balance(&from);
        let to_amount = self.balance(&to);

        let new_from_amount = (from_amount)
            .checked_sub(&amount)
            .ok_or("Insufficient fund")?;

        println!("{:#?}", new_from_amount.clone());

        let new_to_amount = (to_amount)
            .checked_add(&amount)
            .ok_or("Overflow when adding to balance")?;

        self.set_balance(&from, new_from_amount);
        self.set_balance(&to, new_to_amount);

        return Ok(());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_balances() {
        let mut balances = Pallet::new();
        assert_eq!(balances.balance(&"A".to_string()), 0);
        balances.set_balance(&"A".to_string(), 100);

        assert_eq!(balances.balance(&"A".to_string()), 100);
    }

    #[test]
    fn transfer_balance() {
        let a = "A".to_string();
        let b = "B".to_string();
        let mut balances = Pallet::new();
        balances.set_balance(&a, 100);
        let _ = balances.transfer(a.clone(), b.clone(), 50);

        assert_eq!(balances.balance(&a), 50);
        assert_eq!(balances.balance(&b), 50);
    }

    #[test]
    fn transfer_insufficient_balance() {
        let a = "A".to_string();
        let b = "B".to_string();
        let mut balances = Pallet::new();
        balances.set_balance(&a, 100);
        let result = balances.transfer(a.clone(), b.clone(), 200);

        assert_eq!(result, Err("Insufficient fund"));
        assert_eq!(balances.balance(&a), 100);
        assert_eq!(balances.balance(&b), 0);
    }

    #[test]
    fn transfer_overflow_balance() {
        let a = "A".to_string();
        let b = "B".to_string();
        let mut balances = Pallet::new();
        balances.set_balance(&a, u128::MAX);
        balances.set_balance(&b, 100);
        let result = balances.transfer(b.clone(), a.clone(), 50);

        assert_eq!(result, Err("Overflow when adding to balance"));
        assert_eq!(balances.balance(&a), u128::MAX);
        assert_eq!(balances.balance(&b), 100);
    }
}
