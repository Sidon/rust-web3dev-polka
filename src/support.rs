/// Cabeçalho simplificado de um bloco, contendo apenas o número do bloco.
/// Pode ser expandido para incluir mais metadados.
#[derive(Debug)]
pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
}

/// Um extrínseco representa uma transição de estado na blockchain.
/// Inclui o chamador (`caller`) e a chamada específica (`call`).
#[derive(Debug)]  // Adicione isso
pub struct Extrinsic<Caller, Call> {
    pub caller: Caller,
    pub call: Call
}

/// Representação de um bloco na blockchain, contendo um cabeçalho e extrínsecos.
#[derive(Debug)]  // Adicione isso
pub struct Block<Header, Extrinsic> {
    pub header: Header,
    pub extrinsics: Vec<Extrinsic>,
}


/// Tipo de resultado para operações de despacho. Retorna `Ok(())` em caso de sucesso,
/// ou uma mensagem de erro estática em caso de falha.
pub type DispatchResult = Result<(), &'static str>;

/// Trait que define como despachar uma chamada de um chamador específico.
pub trait Dispatch {
    type Caller;
    type Call;

    /// Despacha uma chamada em nome de um chamador.
    /// Retorna `DispatchResult` com base no sucesso ou falha da operação.
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}
