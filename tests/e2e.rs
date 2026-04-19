use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("abs").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("abs"))
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Commands:"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("abs").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("abs 1.1.0")); 
}

#[test]
fn test_auth_subcommands_exist() {
    let mut cmd = Command::cargo_bin("abs").unwrap();
    cmd.arg("auth").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("login"))
        .stdout(predicate::str::contains("logout"));
}

#[test]
fn test_completion_subcommand_exist() {
    let mut cmd = Command::cargo_bin("abs").unwrap();
    cmd.arg("completion").arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "Generate shell completion scripts",
    ));
}

#[test]
fn test_items_subcommands_exist() {
    let mut cmd = Command::cargo_bin("abs").unwrap();
    cmd.arg("items").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("bulk-update"))
        .stdout(predicate::str::contains("match"))
        .stdout(predicate::str::contains("unmatch"));
}

#[test]
fn test_info_command_structure() {
    let mut cmd = Command::cargo_bin("abs").unwrap();
    cmd.arg("info").arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "Get server status and information",
    ));
}
