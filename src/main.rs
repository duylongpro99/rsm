fn main() {
    let mut runtime = rsm::Runtime::new();

    let userA = String::from("A");
    let userB = String::from("B");
    let userC = String::from("C");

    runtime.balances.set_balance(&userA, 100);
    runtime.system.inc_block_number();

    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&userA);

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
