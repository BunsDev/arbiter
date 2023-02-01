use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, Output, TransactTo, B160, U256 as rU256},
    EVM,
};
use bytes::Bytes;

#[derive(Debug, Default)]
pub struct ExecutionManager {
    pub evm: EVM<CacheDB<EmptyDB>>,
}

impl ExecutionManager {
    /// Public constructor function to instantiate an `ExecutionManager`.
    pub fn new() -> Self {
    }

    pub fn execute(
        transact_to: 
        value: rU265
        caller: 
        calldata: Bytes,
    ) -> {

    }
}
