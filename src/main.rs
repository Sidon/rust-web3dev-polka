// Importando os módulos e tipos necessários
mod balances;
mod system;
mod types;
mod traits;


use crate::types::{AccountId, Balance, BlockNumber, RuntimeCall, MyRuntimeTypes, DispatchResult};
use crate::traits::{SupportTypes, Dispatch};

// Estrutura principal do runtime
#[derive(Debug)]
pub struct Runtime<T: SupportTypes> {
    balances: balances::Pallet<T>,
    system: system::Pallet<T>,
}

// Implementação de Dispatch para Runtime<T>
impl<T: SupportTypes> Dispatch for Runtime<T> {
    type Caller = T::AccountId;
    type Call = RuntimeCall<T>;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> DispatchResult {
        match runtime_call {
            RuntimeCall::Transfer { to, amount } => {
                self.transfer(caller, to, amount)
            },
            // Outros casos de RuntimeCall serão tratados aqui
        }
    }
}

impl<T: SupportTypes> Runtime<T> {
    pub fn new() -> Self {
        Self {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.set_balance(who, amount)
    }

    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        self.balances.get_balance(who)
    }

    pub fn transfer(&mut self, caller: T::AccountId, to: T::AccountId, amount: T::Balance) -> Result<(), &'static str> {
        self.balances.transfer(caller, to, amount)
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.system.block_number()
    }

    pub fn inc_block_number(&mut self) {
        self.system.increment_block_number()
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        self.system.update_nonce(who.clone(), self.system.get_nonce(who).unwrap_or_else(|| T::Balance::zero()) + T::Balance::one());
    }

    pub fn execute_block(&mut self, block: T::Block) -> DispatchResult {
        // 1. Incrementar o número do bloco do sistema.
        self.inc_block_number();
        println!("Executando bloco: {:?}", self.block_number());

        // 2. Processar cada extrínseco no bloco.
        for (i, extrinsic) in block.extrinsics.iter().enumerate() {
            let caller = &extrinsic.caller;
            let call = &extrinsic.call;

            match self.dispatch(caller.clone(), call.clone()) {
                Ok(_) => println!("Extrínseco {} executado com sucesso", i),
                Err(e) => {
                    println!("Erro de Extrínseco\n\tNúmero do Bloco: {}\n\tNúmero do Extrínseco: {}\n\tErro: {}",
                             block.header.number, i, e);
                },
            }
        }

        Ok(())
    }
}

fn debug_structures() {
    // Exemplo de criação e debug de um RuntimeCall
    let call = RuntimeCall::Transfer {
        to: "bob".to_string(),
        amount: 50,
    };
    println!("RuntimeCall: {:?}", call);

    // Exemplo de debug de MyRuntimeTypes
    let runtime_types = MyRuntimeTypes;
    println!("MyRuntimeTypes: {:?}", runtime_types);

    // Criando um header e um extrinsic para um bloco
    let header = crate::types::Header {
        parent_hash: "0xparent".to_string(),
        number: BlockNumber(1),
        state_root: "0xstate".to_string(),
        extrinsics_root: "0xextrinsics".to_string(),
    };
    let extrinsic = crate::types::Extrinsic {
        signature: "0xsignature".to_string(),
        call,
    };
    let block = crate::types::Block {
        header,
        extrinsics: vec![extrinsic],
    };
    println!("Block: {:?}", block);
}

fn main() {
    // Chama a função de debug para exibir as estruturas
    debug_structures();

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
    runtime.transfer(alice.clone(), bob.clone(), Balance(30)).unwrap();

    // Transferência de 20 de Alice para Charlie
    runtime.transfer(alice.clone(), AccountId("charlie".to_string()), Balance(20)).unwrap();

    println!("Resultado:");
    println!("Saldo de Alice: {}", runtime.balance(&alice));
    println!("Saldo de Bob: {}", runtime.balance(&bob));

    println!("{:#?}", runtime);
}
