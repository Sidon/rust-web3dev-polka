// traits.rs
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{AddAssign, Mul, SubAssign};
use num::{One, Zero};
use crate::support::Block;
// Importando os tipos necessários
use crate::types::{Header, Extrinsic, DispatchResult, MyRuntimeTypes};

// Definição da trait BlockchainTypes
pub trait BlockchainTypes {
    type AccountId: Ord + Debug + Clone + Hash + Eq;
    type BlockNumber: Copy + Debug + One + Mul<Output = Self::BlockNumber> + Zero + AddAssign;
    type Balance: Copy + Debug + PartialOrd + Zero + AddAssign + SubAssign + One;
}


// Definição da trait SupportTypes que estende PalletTypes
pub trait SupportTypes: BlockchainTypes {
    type Extrinsic;
    type Header;
    type Block;
}


/// Trait que define como despachar uma chamada de um chamador específico.
pub trait Dispatch {
    type Caller;
    type Call;

    /// Despacha uma chamada em nome de um chamador.
    /// Retorna `DispatchResult` com base no sucesso ou falha da operação.
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}

impl BlockchainTypes for MyRuntimeTypes {
    type AccountId = String;
    type BlockNumber = u32;
    type Balance = u64; // u128 já implementa `SubAssign`, então isso deveria funcionar
}


impl SupportTypes for MyRuntimeTypes {
    type Extrinsic = Extrinsic;
    type Header = Header;
    type Block = Block<Self::Header, Self::Extrinsic>;
}

