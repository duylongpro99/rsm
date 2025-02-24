use crate::{balances, proof_of_existence, support, Runtime};

pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
}

pub type AccountId = String;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Nonce = u32;
pub type Content = &'static str;

pub type Extrinsic = support::Extrinsic<AccountId, RuntimeCall>;
pub type Header = support::Header<BlockNumber>;
pub type Block = support::Block<Header, Extrinsic>;
