use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    about = "CLI tool to add a file to IPFS and store the CID in a dedicated smart contract"
)]
pub struct Configuration {
    /// Input file to be sent to IPFS
    #[structopt(long, parse(from_os_str))]
    pub input_file: PathBuf,
    /// Contract name
    #[structopt(long, default_value = "CidStorage")]
    pub contract_name: String,
    /// Contract file path
    #[structopt(long, parse(from_os_str))]
    pub contract_path: PathBuf,
    /// Account address
    #[structopt(long)]
    pub account_address: String,
    /// Contract address
    #[structopt(long)]
    pub contract_address: String,
    /// Ganache address
    #[structopt(long, default_value = "http://localhost:8545")]
    pub ganache_address: String,
}
