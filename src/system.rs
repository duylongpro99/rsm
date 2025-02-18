use std::{collections::BTreeMap, ops::AddAssign};

use num::traits::{CheckedAdd, CheckedSub, One, Zero};

pub trait Config {
    type BlockNumber: Clone + Zero + Copy + One + AddAssign + CheckedAdd + CheckedSub;
    type AccountId: Ord + Clone;
    type Nonce: Copy + Zero + One;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Pallet {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number = self
            .block_number
            .checked_add(&T::BlockNumber::one())
            .unwrap();
    }

    pub fn inc_nonce(&mut self, to: &T::AccountId) {
        let nonce = *self.nonce.get(&to).unwrap_or(&T::Nonce::zero());
        self.nonce.insert(to.clone(), nonce + T::Nonce::one());
    }

    pub fn get_nonce(&self, user: &T::AccountId) -> T::Nonce {
        *self.nonce.get(user).unwrap_or(&T::Nonce::zero())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestConfig {}

    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let system: Pallet<TestConfig> = Pallet::new();
        assert_eq!(system.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut system: Pallet<TestConfig> = Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number(), 1);
    }

    #[test]
    fn inc_nonce() {
        let mut system: Pallet<TestConfig> = Pallet::new();
        let user = String::from("A");
        system.inc_nonce(&user);
        assert_eq!(system.get_nonce(&user), 1);
    }
}
