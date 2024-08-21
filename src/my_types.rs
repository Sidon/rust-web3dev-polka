use num::{Zero, One, CheckedAdd, CheckedSub};
use std::ops::AddAssign;
use std::fmt::Debug;
use std::clone::Clone;
use std::hash::Hash;


/// Implementação concreta de `PalletTypes` para nosso runtime.
#[derive(Debug)]
pub struct MyRuntimeTypes;


/// Configura os tipos fundamentais usados nos pallets.
pub trait PalletTypes {
    type AccountId: Ord + Debug + Clone + Hash + Eq;
    type BlockNumber: Zero + One + AddAssign + Copy + Debug;
    type Balance: Zero + One + AddAssign + Copy + Debug + CheckedAdd + CheckedSub;
}

/// Extensão de `PalletTypes` para incluir tipos relacionados ao suporte da blockchain.
pub trait SupportTypes: PalletTypes {
    type Extrinsic;
    type Header;
    type Block;
}

/// Implementação de `SupportTypes` para `MyRuntimeTypes`.
impl SupportTypes for MyRuntimeTypes {
    type Extrinsic = crate::support::Extrinsic<Self::AccountId, RuntimeCall>;
    type Header = crate::support::Header<Self::BlockNumber>;
    type Block = crate::support::Block<Self::Header, Self::Extrinsic>;
}

impl PalletTypes for MyRuntimeTypes {
    type AccountId = String;
    type BlockNumber = u32;
    type Balance = u128;
}
