use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "contract-deploy",
    about = "Helper utility to upload the CidStorage smart contract."
)]
pub struct Opt {
    /// Contract name
    #[structopt(long, default_value = "CidStorage")]
    pub contract_name: String,
    /// Contract file path
    #[structopt(long, parse(from_os_str))]
    pub contract_path: PathBuf,
    /// Account address
    #[structopt(long)]
    pub account_address: String,
    /// Ganache address
    #[structopt(long, default_value = "http://localhost:8545")]
    pub ganache_address: String,
}
