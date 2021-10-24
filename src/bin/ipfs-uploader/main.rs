mod config;

use structopt::StructOpt;

use std::io::BufReader;

use crate::config::Configuration;
use ethers::abi::ethereum_types::H256;
use ethers::abi::Address;
use ethers::contract::Contract;
use ethers::prelude::{Http, Provider};
use ifpfs_uploader::utils::contract::compile_contract;
use ipfs_api_backend_hyper::response::AddResponse;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use log::{debug, info};
use std::convert::TryFrom;
use std::fs::File;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = config::Configuration::from_args();
    env_logger::init();
    debug!("{:?}", opt);

    let ipfs_result = upload_file_to_ipfs(&opt.input_file).await;
    info!("Uploaded file to IPFS with CID: {}", ipfs_result.hash);

    let contract = get_contract(&opt);

    let current_cid: String = contract.method::<_, String>("retrieve", ())?.call().await?;
    info!("Old CID: {}", &current_cid);

    if current_cid == ipfs_result.hash {
        info!("Same CID already set, quitting early.");
        println!("{}", current_cid);
        return Ok(());
    }

    let store_call = contract
        .method::<_, H256>("store", ipfs_result.hash.clone())?
        .legacy();
    let pending_tx = store_call.send().await?;
    info!("New CID: {}", &ipfs_result.hash);

    let receipt = pending_tx.await?;
    info!(
        "Upload completed, transaction hash: {}",
        receipt.unwrap().transaction_hash
    );

    println!("{}", ipfs_result.hash);
    Ok(())
}

/// Gets contract handle
/// Fails with generic error.
fn get_contract(opt: &Configuration) -> Contract<Provider<Http>> {
    let (abi, _) = compile_contract(&opt.contract_name, &opt.contract_path);
    let account_address = opt
        .account_address
        .parse::<Address>()
        .expect("Could not parse account address");
    let client = Provider::<Http>::try_from(opt.ganache_address.clone())
        .expect("Could not parse ganache address")
        .with_sender(account_address);
    let contract_address = opt
        .contract_address
        .parse::<Address>()
        .expect("Could not parse contract address");
    Contract::new(contract_address, abi, client)
}

/// Uploads given file to IPFS
/// Returns response from IPFS, generic error otherwise.
async fn upload_file_to_ipfs(input_file: &Path) -> AddResponse {
    let client = IpfsClient::default();
    let input = BufReader::new(File::open(input_file).expect("Could not open input file"));
    client
        .add(input)
        .await
        .expect("Could not add the file to IPFS")
}
