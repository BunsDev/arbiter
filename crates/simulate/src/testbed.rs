use revm::{
    db::{CacheDB, EmptyDB},
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

    pub fn execute() -> {

    }
}
