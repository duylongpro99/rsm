use crate::{support, RuntimeCall};

pub type AccountId = String;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Nonce = u32;
pub type Content = &'static str;

pub type Extrinsic = support::Extrinsic<AccountId, RuntimeCall>;
pub type Header = support::Header<BlockNumber>;
pub type Block = support::Block<Header, Extrinsic>;
