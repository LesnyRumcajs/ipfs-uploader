use std::path::Path;

use ethers::core::abi::Abi;
use ethers::core::types::Bytes;
use ethers::core::utils::Solc;

use log::debug;

/// compiles the given contract and returns the ABI and Bytecode
pub fn compile_contract(name: &str, filename: &Path) -> (Abi, Bytes) {
    debug!(
        "Compiling contract {} from {}",
        name,
        filename.to_str().unwrap()
    );
    let compiled = Solc::new(filename.to_str().unwrap()).build().unwrap();
    let contract = compiled.get(name).expect("could not find contract");
    (contract.abi.clone(), contract.bytecode.clone())
}
