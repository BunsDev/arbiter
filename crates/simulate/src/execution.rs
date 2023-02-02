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
        let mut evm = EVM::new();
        let mut db = CacheDB::new(EmptyDB {});

        db.insert_account_info(uniswap, AccountInfo {
            balance: U256::zero(),
            code: Bytes..
            nonce: 0
            storage
        })
        evm.database(db);

        Self { evm }
    }



    

    // pub fn execute(
    //     transact_to: 
    //     value: rU265
    //     caller: 
    //     calldata: Bytes,
    // ) -> {

    // }
}
