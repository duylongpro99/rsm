use std::{collections::BTreeMap, fmt::Debug};

use crate::{support::DispatchResult, system};

pub trait Config: system::Config {
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.get_claim(&claim) {
            Some(_) => Err("Claim already existed"),
            None => {
                self.claims.insert(claim, caller);
                Ok(())
            }
        }
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let owner = self.get_claim(&claim).ok_or("Claim does not existed")?;
        if owner != &caller {
            return Err("Caller is not the owner of claim");
        }

        self.claims.remove(&claim);
        Ok(())
    }
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }
}

#[cfg(test)]
mod test {
    use crate::types;

    use super::*;

    struct TestConfig;

    impl system::Config for TestConfig {
        type AccountId = types::AccountId;
        type BlockNumber = types::BlockNumber;
        type Nonce = types::Nonce;
    }

    impl Config for TestConfig {
        type Content = &'static str;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut poe: Pallet<TestConfig> = Pallet::new();
        assert_eq!(poe.get_claim(&"A"), None);
        poe.create_claim("A".to_string(), "transactionA").unwrap();
        assert_eq!(poe.get_claim(&"transactionA"), Some(&"A".to_string()));

        let res = poe.create_claim("B".to_string(), "transactionA");
        assert_eq!(res, Err("Claim already existed"));

        poe.revoke_claim("A".to_string(), "transactionA").unwrap();
        assert_eq!(poe.get_claim(&"transactionA"), None);
    }
}
