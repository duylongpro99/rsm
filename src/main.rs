use support::Runtime;
use types::RuntimeCall;

mod balances;
mod proof_of_existence;
mod support;
mod system;
mod types;

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
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: String::from("B"),
                    amount: 50,
                }),
            },
            support::Extrinsic {
                caller: b.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
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
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    claim: "transactionA",
                }),
            },
            support::Extrinsic {
                caller: b.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    claim: "transactionB",
                }),
            },
        ],
    };

    runtime.execute_block(block1).expect("Cannot execute block")
}
