// Importando os tipos e traits necessários
use crate::traits::BlockchainTypes;
use std::collections::BTreeMap;
use std::ops::SubAssign;
use num::Zero;

// Definição da Pallet para o módulo de Balances
#[derive(Debug)]
pub struct Pallet<T: BlockchainTypes> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: BlockchainTypes> Pallet<T> {
    pub fn new() -> Self {
        Pallet {
            balances: BTreeMap::new(),
        }
    }
}


impl<T: BlockchainTypes> Pallet<T> {
    // Função para consultar o saldo de uma conta
    pub fn get_balance(&self, account: &T::AccountId) -> Option<&T::Balance> {
        self.balances.get(account)
    }

    // Função para atualizar o saldo de uma conta
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    // Função para realizar transferência entre contas
    pub fn transfer(&mut self, from: &T::AccountId, to: &T::AccountId, amount: T::Balance) -> Result<(), &'static str> where <T as BlockchainTypes>::Balance: SubAssign {
        let from_balance = self.balances.get_mut(from).ok_or("Sender account not found")?;
        if *from_balance < amount {
            return Err("Insufficient balance");
        }
        *from_balance -= amount;
        let to_balance = self.balances.entry(to.clone()).or_insert(T::Balance::zero());
        *to_balance += amount;
        Ok(())
    }
}
