use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct Configuration {
    /// Input file to be sent to IPFS
    #[structopt(parse(from_os_str))]
    pub input_file: PathBuf,
}
