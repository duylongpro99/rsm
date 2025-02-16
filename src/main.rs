mod balances;
mod system;
mod types;

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type AccountId = types::AccountId;
    type Balance = types::Balance;
}
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();

    let userA = String::from("A");
    let userB = String::from("B");
    let userC = String::from("C");

    runtime.balances.set_balance(&userA, 100);
    runtime.system.inc_block_number();

    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(userA.clone());

    let _ = runtime
        .balances
        .transfer(userA.clone(), userB.clone(), 30)
        .map_err(|e| println!("Error: {}", e));

    let _ = runtime
        .balances
        .transfer(userA.clone(), userC.clone(), 20)
        .map_err(|e| println!("Error: {}", e));

    println!("{:#?}", runtime);
}
