use ethers::abi::Address;
use ethers::contract::ContractFactory;
use ethers::prelude::{Http, Provider};
use ifpfs_uploader::utils::contract::compile_contract;
use log::debug;
use std::convert::TryFrom;
use std::sync::Arc;
use structopt::StructOpt;

mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = config::Opt::from_args();
    env_logger::init();
    debug!("{:?}", opt);

    let (abi, bytecode) = compile_contract(&opt.contract_name, &opt.contract_path);
    let account_address = opt.account_address.parse::<Address>()?;
    let client =
        Arc::new(Provider::<Http>::try_from(opt.ganache_address)?.with_sender(account_address));
    let factory = ContractFactory::new(abi, bytecode, client);

    debug!(
        "Deploying contract with account address: {}",
        account_address
    );
    let deployer = factory.deploy(())?.legacy();
    let contract = deployer.clone().send().await?;
    println!("{:?}", contract.address());

    Ok(())
}
