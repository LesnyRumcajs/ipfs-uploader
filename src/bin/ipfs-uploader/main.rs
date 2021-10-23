mod config;
use structopt::StructOpt;

use log::debug;
fn main() {
    let opt = config::Configuration::from_args();
    env_logger::init();
    debug!("{:?}", opt);
}
