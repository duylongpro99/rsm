use support::Runtime;
use types::RuntimeCall;

mod balances;
mod support;
mod system;
mod types;
mod proof_of_existence;

fn main() {
    let a = String::from("A");
    let mut runtime = Runtime::new();
    runtime.balances.set_balance(&a, 100);
    let block = support::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![support::Extrinsic {
            caller: a.clone(),
            call: RuntimeCall::Balances(balances::Call::Transfer {
                to: String::from("B"),
                amount: 50,
            }),
        }],
    };

    runtime.execute_block(block).expect("Cannot execute block");
}
