mod balances;
mod proof_of_existence;
mod support;
mod system;
mod types;

use support::Dispatch;

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    pub system: system::Pallet<Runtime>,
    pub balances: balances::Pallet<Runtime>,
    pub proof_of_existence: proof_of_existence::Pallet<Runtime>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

fn main() {
    let a: String = String::from("A");
    let b: String = String::from("B");
    let mut runtime = Runtime::new();
    runtime.balances.set_balance(&a, 100);
    let block = support::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: a.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: String::from("B"),
                    amount: 50,
                }),
            },
            support::Extrinsic {
                caller: b.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: String::from("B"),
                    amount: 50,
                }),
            },
        ],
    };

    runtime.execute_block(block).expect("Cannot execute block");

    let block1 = support::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: a.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "transactionA",
                }),
            },
            support::Extrinsic {
                caller: b.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "transactionB",
                }),
            },
        ],
    };

    runtime.execute_block(block1).expect("Cannot execute block")
}
