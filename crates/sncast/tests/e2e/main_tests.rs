use crate::helpers::constants::{ACCOUNT, ACCOUNT_FILE_PATH, CONTRACTS_DIR, URL};
use crate::helpers::fixtures::{duplicate_directory_with_salt, from_env, get_keystores_path};
use crate::helpers::runner::runner;
use indoc::indoc;
use snapbox::cmd::{cargo_bin, Command};
use sncast::helpers::constants::KEYSTORE_PASSWORD_ENV_VAR;
use std::env;
use std::fs;

#[tokio::test]
async fn test_happy_case_from_scarb() {
    let args = vec![
        "--accounts-file",
        ACCOUNT_FILE_PATH,
        "--path-to-scarb-toml",
        "tests/data/files/correct_Scarb.toml",
        "call",
        "--contract-address",
        "0x0",
        "--function",
        "doesnotmatter",
    ];

    let snapbox = runner(&args);

    snapbox.assert().success().stderr_matches(indoc! {r"
        command: call
        error: There is no contract at the specified address
    "});
}

#[tokio::test]
async fn test_happy_case_from_cli_no_scarb() {
    let args = vec![
        "--accounts-file",
        ACCOUNT_FILE_PATH,
        "--url",
        URL,
        "--account",
        ACCOUNT,
        "call",
        "--contract-address",
        "0x0",
        "--function",
        "doesnotmatter",
    ];

    let snapbox = runner(&args);

    snapbox.assert().success().stderr_matches(indoc! {r"
        command: call
        error: There is no contract at the specified address
    "});
}

#[tokio::test]
async fn test_happy_case_from_cli_with_scarb() {
    let address = from_env("CAST_MAP_ADDRESS").unwrap();
    let args = vec![
        "--accounts-file",
        ACCOUNT_FILE_PATH,
        "--path-to-scarb-toml",
        "tests/data/files/correct_Scarb.toml",
        "--profile",
        "profile1",
        "--url",
        URL,
        "--account",
        ACCOUNT,
        "call",
        "--contract-address",
        &address,
        "--function",
        "get",
        "--calldata",
        "0x0",
        "--block-id",
        "latest",
    ];

    let snapbox = runner(&args);

    snapbox.assert().success().stdout_eq(indoc! {r"
        command: call
        response: [0x0]
    "});
}

#[tokio::test]
async fn test_happy_case_mixed() {
    let address = from_env("CAST_MAP_ADDRESS").unwrap();
    let args = vec![
        "--accounts-file",
        ACCOUNT_FILE_PATH,
        "--path-to-scarb-toml",
        "tests/data/files/correct_Scarb.toml",
        "--profile",
        "profile2",
        "--account",
        ACCOUNT,
        "call",
        "--contract-address",
        &address,
        "--function",
        "get",
        "--calldata",
        "0x0",
        "--block-id",
        "latest",
    ];

    let snapbox = runner(&args);

    snapbox.assert().success().stdout_eq(indoc! {r"
        command: call
        response: [0x0]
    "});
}

#[tokio::test]
async fn test_missing_account() {
    let args = vec![
        "--accounts-file",
        ACCOUNT_FILE_PATH,
        "--url",
        URL,
        "declare",
        "--contract-name",
        "whatever",
    ];

    let snapbox = runner(&args);

    snapbox.assert().stderr_matches(indoc! {r"
        Error: Account name not passed nor found in Scarb.toml
    "});
}

#[tokio::test]
async fn test_missing_url() {
    let args = vec![
        "--accounts-file",
        ACCOUNT_FILE_PATH,
        "--account",
        ACCOUNT,
        "declare",
        "--contract-name",
        "whatever",
    ];

    let snapbox = runner(&args);

    snapbox.assert().stderr_matches(indoc! {r"
        Error: RPC url not passed nor found in Scarb.toml
    "});
}

#[tokio::test]
async fn test_inexistent_keystore() {
    let args = vec![
        "--url",
        URL,
        "--keystore",
        "inexistent_key.json",
        "declare",
        "--contract-name",
        "my_contract",
    ];

    let snapbox = runner(&args);

    snapbox.assert().stderr_matches(indoc! {r"
        Error: Failed to find keystore file
    "});
}

#[tokio::test]
async fn test_keystore_account_required() {
    let args = vec![
        "--url",
        URL,
        "--keystore",
        "tests/data/keystore/my_key.json",
        "declare",
        "--contract-name",
        "my_contract",
    ];

    let snapbox = runner(&args);

    snapbox.assert().stderr_matches(indoc! {r"
        Error: Passed empty path for `--account`
    "});
}

#[tokio::test]
async fn test_keystore_inexistent_account() {
    let args = vec![
        "--url",
        URL,
        "--keystore",
        "tests/data/keystore/my_key.json",
        "--account",
        "inexistent_account",
        "declare",
        "--contract-name",
        "my_contract",
    ];

    let snapbox = runner(&args);

    snapbox.assert().stderr_matches(indoc! {r"
        Error: File containing the account does not exist[..]
    "});
}

#[tokio::test]
async fn test_keystore_undeployed_account() {
    let contract_path =
        duplicate_directory_with_salt(CONTRACTS_DIR.to_string() + "/map", "put", "8");
    let my_key_path = get_keystores_path("tests/data/keystore/my_key.json");
    let my_account_undeployed_path =
        get_keystores_path("tests/data/keystore/my_account_undeployed.json");

    let args = vec![
        "--url",
        URL,
        "--keystore",
        my_key_path.as_str(),
        "--account",
        my_account_undeployed_path.as_str(),
        "declare",
        "--contract-name",
        "Map",
    ];

    env::set_var(KEYSTORE_PASSWORD_ENV_VAR, "123");
    let snapbox = Command::new(cargo_bin!("sncast"))
        .current_dir(contract_path.path())
        .args(args);

    snapbox.assert().stderr_matches(indoc! {r"
        Error: [..] make sure the account is deployed
    "});

    fs::remove_dir_all(contract_path).unwrap();
}

#[tokio::test]
async fn test_keystore_declare() {
    let contract_path =
        duplicate_directory_with_salt(CONTRACTS_DIR.to_string() + "/map", "put", "999");
    let my_key_path = get_keystores_path("tests/data/keystore/predeployed_key.json");
    let my_account_path = get_keystores_path("tests/data/keystore/predeployed_account.json");
    let args = vec![
        "--url",
        URL,
        "--keystore",
        my_key_path.as_str(),
        "--account",
        my_account_path.as_str(),
        "declare",
        "--contract-name",
        "Map",
    ];

    env::set_var(KEYSTORE_PASSWORD_ENV_VAR, "123");
    let snapbox = Command::new(cargo_bin!("sncast"))
        .current_dir(contract_path.path())
        .args(args);

    assert!(snapbox.assert().success().get_output().stderr.is_empty());
}
