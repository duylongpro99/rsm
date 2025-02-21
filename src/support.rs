use crate::{balances, system, types};

pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
}

pub struct Extrinsic<Caller, Call> {
    pub caller: Caller,
    pub call: Call,
}

pub struct Block<Header, Extrinsic> {
    pub header: Header,
    pub extrinsics: Vec<Extrinsic>,
}

pub type DispatchResult = Result<(), &'static str>;

pub trait Dispatch {
    type Caller;
    type Call;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}

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

    pub fn execute_block(&mut self, block: types::Block) -> DispatchResult {
        self.system.inc_block_number();

        if self.system.block_number() != block.header.block_number {
            return Err("Block number mismatch");
        }

        for (idx, Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);
            let _ = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error \n\t Block Number {}\n\t Extrinsic Number {} \n\t Error: {}",
                    block.header.block_number, idx, e
                )
            });
        }

        Ok(())
    }
}

impl Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = types::RuntimeCall;

    fn dispatch(&mut self, caller: Self::Caller, runtime_call: Self::Call) -> DispatchResult {
        match runtime_call {
            types::RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}
