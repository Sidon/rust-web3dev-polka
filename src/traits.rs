// traits.rs
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{AddAssign, Mul};
use num::{One, Zero};
// Importando os tipos necessários
use crate::types::{AccountId, BlockNumber, Balance, Header, Block, Extrinsic, DispatchResult};

// Definição da trait BlockchainTypes
pub trait BlockchainTypes {
    type AccountId: Ord + Debug + Clone + Hash + Eq;
    type BlockNumber: Copy + Debug + One + Mul<Output = Self::BlockNumber> + Zero + AddAssign;
    type Balance: Copy + Debug + PartialOrd + Zero + AddAssign;
}


// Implementação padrão de PalletTypes para os tipos do sistema
impl BlockchainTypes for () {
    type AccountId = AccountId;
    type BlockNumber = BlockNumber;
    type Balance = Balance;
}

// Definição da trait SupportTypes que estende PalletTypes
pub trait SupportTypes: BlockchainTypes {
    type Extrinsic;
    type Header;
    type Block;
}

// Implementação padrão de SupportTypes para os tipos do sistema
impl SupportTypes for () {
    type Extrinsic = Extrinsic;
    type Header = Header;
    type Block = Block;
}

/// Trait que define como despachar uma chamada de um chamador específico.
pub trait Dispatch {
    type Caller;
    type Call;

    /// Despacha uma chamada em nome de um chamador.
    /// Retorna `DispatchResult` com base no sucesso ou falha da operação.
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}

