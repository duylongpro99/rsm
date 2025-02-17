
mod balances;
mod system;
mod types;

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

#[derive(Debug)]
pub struct Runtime {
    pub system: system::Pallet<Runtime>,
    pub balances: balances::Pallet<Runtime>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}
