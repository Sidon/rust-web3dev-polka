// Importando os tipos e traits necessários
use crate::traits::BlockchainTypes;
use std::collections::BTreeMap;
use num::{Zero, One};


// Definição da Pallet para o módulo de Sistema
#[derive(Debug)]
pub struct Pallet<T: BlockchainTypes> {
    pub block_number: T::BlockNumber,
    pub nonce: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: BlockchainTypes> Pallet<T> {

    pub fn new() -> Self {
        Pallet {
            block_number: T::BlockNumber::zero(), // Inicializando com zero usando a trait Zero
            nonce: BTreeMap::new(), // Inicializando o BTreeMap vazio para nonce
        }
    }

    // Função para incrementar o número do bloco
    pub fn increment_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    // Função para obter o nonce de uma conta
    pub fn get_nonce(&self, account: &T::AccountId) -> Option<&T::Balance> {
        self.nonce.get(account)
    }

    // Função para atualizar o nonce de uma conta
    pub fn update_nonce(&mut self, account: T::AccountId, nonce: T::Balance) {
        self.nonce.insert(account, nonce);
    }
}
