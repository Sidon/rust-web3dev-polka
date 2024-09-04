// Definição de tipos fundamentais

use std::ops::{Add, AddAssign, Deref, Mul, SubAssign};
use num::{One, Zero};
use crate::traits::SupportTypes;

// Tipo para identificadores de conta
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct AccountId(pub String);

// Tipo para números de bloco
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct BlockNumber(pub u64);

// Tipo para saldo das contas
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Balance(pub u64);


/// Define as chamadas disponíveis no Runtime.
#[derive(Debug, Clone)]
pub enum RuntimeCall<T: SupportTypes> {
    Transfer { to: T::AccountId, amount: T::Balance },
    // outras variantes
}



// Tipo para cabeçalhos de bloco
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub parent_hash: String,
    pub number: BlockNumber,
    pub state_root: String,
    pub extrinsics_root: String,
}


// Tipo para extrínsecos (transações/comandos)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extrinsic {
    pub signature: String, // Isso pode ser mais complexo, dependendo da implementação
    pub call: String,      // Representa o que o extrínseco faz (pode ser um enum ou algo mais detalhado)
}

#[derive(Debug)]
pub struct MyRuntimeTypes;

/// Tipo de resultado para operações de despacho. Retorna `Ok(())` em caso de sucesso,
/// ou uma mensagem de erro estática em caso de falha.
pub type DispatchResult = Result<(), &'static str>;


impl Deref for AccountId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl Add for Balance {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Balance(self.0 + other.0)
    }
}

impl AddAssign for Balance {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl AddAssign for BlockNumber {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Zero for Balance {
    fn zero() -> Self {
        Balance(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}


impl Mul for BlockNumber {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        BlockNumber(self.0 * other.0)
    }
}


impl One for BlockNumber {
    fn one() -> Self {
        BlockNumber(1)
    }
}




// Implementação da trait Mul para Balance
impl Mul for Balance {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Balance(self.0 * rhs.0)
    }
}


// Implementa a trait One para o tipo Balance
impl One for Balance {
    fn one() -> Self {
        Balance(1)
    }
}

impl Add for BlockNumber {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        BlockNumber(self.0 + other.0)
    }

}

impl Zero for BlockNumber {
    fn zero() -> Self {
        BlockNumber(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

// Implementa SubAssign para Balance
impl SubAssign for Balance {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}