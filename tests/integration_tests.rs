use assert_cmd::Command;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::io::Write;
use tempfile::NamedTempFile;

// Using pre-determined accounts for ganache (-d switch)
static GANACHE_ACCOUNT_1: &str = "0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1";
static GANACHE_ACCOUNT_2: &str = "0xFFcf8FDEE72ac11b5c542428B35EEF5769C409f0";

#[test]
fn contract_deploy_must_have_required_params() {
    let mut cmd = Command::cargo_bin("contract-deploy").unwrap();
    cmd.assert().failure();
}

#[test]
fn contract_deploy_should_deploy_smart_contract() {
    let cmd = Command::cargo_bin("contract-deploy")
        .unwrap()
        .arg("--account-address")
        .arg(GANACHE_ACCOUNT_1)
        .arg("--contract-path")
        .arg("contracts/CidStorage.sol")
        .assert()
        .success();
    assert!(!cmd.get_output().stdout.is_empty());
}

#[test]
fn ipfs_upload_must_have_required_params() {
    let mut cmd = Command::cargo_bin("ipfs-uploader").unwrap();
    cmd.assert().failure();
}

#[test]
fn ipfs_upload_should_deploy_file_to_ipfs_and_smart_contract() {
    // Generate a valid smart contract
    let deploy_cmd = Command::cargo_bin("contract-deploy")
        .unwrap()
        .arg("--account-address")
        .arg(GANACHE_ACCOUNT_1)
        .arg("--contract-path")
        .arg("contracts/CidStorage.sol")
        .assert()
        .success();

    // Get the contract address
    let contract_address: String = String::from_utf8(deploy_cmd.get_output().stdout.clone())
        .unwrap()
        .trim()
        .to_owned();

    // Generate a random file to upload to IPFS
    let mut temp_file = NamedTempFile::new().unwrap();
    let temp_file_content: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(100)
        .map(char::from)
        .collect();
    temp_file.write_all(temp_file_content.as_bytes()).unwrap();

    // Upload the file to IPFS and ganache as smart contract
    let ipfs_upload_cmd = Command::cargo_bin("ipfs-uploader")
        .unwrap()
        .arg("--input-file")
        .arg(temp_file.path())
        .arg("--account-address")
        .arg(GANACHE_ACCOUNT_2)
        .arg("--contract-path")
        .arg("contracts/CidStorage.sol")
        .arg("--contract-address")
        .arg(contract_address)
        .assert()
        .success();

    // Get the CID of the output
    let cid: String = String::from_utf8(ipfs_upload_cmd.get_output().stdout.clone())
        .unwrap()
        .trim()
        .to_owned();

    // Verify the content in IPFS from the smart contract is what we uploaded
    let ipfs_verify = Command::new("docker")
        .arg("exec")
        .arg("ipfs_host")
        .arg("ipfs")
        .arg("cat")
        .arg(cid)
        .assert()
        .success();

    let actual_content: String =
        String::from_utf8(ipfs_verify.get_output().stdout.clone()).unwrap();
    assert_eq!(actual_content, temp_file_content);
}
