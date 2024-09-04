// Importando os módulos e tipos necessários
mod balances;
mod system;
mod types;
mod traits;
mod support;

use num::{One, Zero};
use crate::types::{AccountId, Balance, RuntimeCall, MyRuntimeTypes, DispatchResult};
use crate::traits::{SupportTypes, Dispatch};

// Estrutura principal do runtime
#[derive(Debug)]
pub struct Runtime<T: SupportTypes> {
    balances: balances::Pallet<T>,
    system: system::Pallet<T>,
}

// Implementação de Dispatch para Runtime<T>
impl<T: SupportTypes<Balance = u64>> Dispatch for Runtime<T> {
    type Caller = T::AccountId;
    type Call = RuntimeCall<T>;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> DispatchResult {
        match runtime_call {
            RuntimeCall::Transfer { to, amount} => {
                self.transfer(caller, to, amount)
            },
            // Outros casos de RuntimeCall serão tratados aqui
        }
    }
}

impl<T: SupportTypes<Balance = u64>> Runtime<T> {
    pub fn new() -> Self {
        Self {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }


    pub fn set_balance(&mut self, who: &T::AccountId, amount: Balance) {
        self.balances.set_balance(who, amount.0)
    }

    pub fn balance(&self, who: &T::AccountId) -> Option<&T::Balance> {
        self.balances.get_balance(who)
    }

    pub fn transfer(&mut self, caller: T::AccountId, to: T::AccountId, amount: u64) -> Result<(), &'static str> {
        self.balances.transfer(&caller, &to, amount)
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.system.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.system.increment_block_number()
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        self.system.update_nonce(
            who.clone(),
            self.system
                .get_nonce(who)
                .copied()
                .unwrap_or_else(T::Balance::zero)
                + T::Balance::one(),
        );
    }
    // pub fn execute_block(&mut self, block: T::Block) -> DispatchResult {
    //
    //     dbg!(std::any::type_name::<T::Block>());
    //
    //     // 1. Incrementar o número do bloco do sistema.
    //     self.inc_block_number();
    //     println!("Executando bloco: {:?}", self.block_number());
    //
    //
    //     // 2. Processar cada extrínseco no bloco.
    //     for (i, extrinsic) in block.extrinsics.iter().enumerate() {
    //         let caller = &extrinsic.caller;
    //         let call = &extrinsic.call;
    //
    //         match self.dispatch(caller.clone(), call.clone()) {
    //             Ok(_) => println!("Extrínseco {} executado com sucesso", i),
    //             Err(e) => {
    //                 println!("Erro de Extrínseco\n\tNúmero do Bloco: {}\n\tNúmero do Extrínseco: {}\n\tErro: {}",
    //                          block.header.number, i, e);
    //             },
    //         }
    //     }
    //
    //     Ok(())
    // }
}


fn main() {

    // Criação do runtime usando MyRuntimeTypes
    let mut runtime = Runtime::<MyRuntimeTypes>::new();

    // Criação dos usuários
    let alice = AccountId("alice".to_string());
    let bob = AccountId("bob".to_string());

    // Inicialização do bloco
    runtime.inc_block_number();

    // Inicialização do nonce de alice
    runtime.inc_nonce(&alice);

    // Primeira transação
    runtime.inc_nonce(&alice);
    runtime.set_balance(&alice, Balance(100));

    // Transferência de 30 de Alice para Bob
    runtime.transfer(alice.0.clone(), bob.0.clone(), 30).unwrap();

    // Transferência de 20 de Alice para Charlie
    runtime.transfer(alice.0.clone(), "charlie".to_string(), 20).unwrap();

    println!("Resultado:");
    println!("Saldo de Alice: {:?}", runtime.balance(&alice));
    println!("Saldo de Bob: {:?}", runtime.balance(&bob));


    println!("{:#?}", runtime);
}
