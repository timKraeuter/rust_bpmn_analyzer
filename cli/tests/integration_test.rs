use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

const P2_PATH: &str = "tests/resources/p2.bpmn";
const P6_STUCK_PATH: &str = "tests/resources/p6_stuck.bpmn";

fn cli() -> assert_cmd::Command {
    let mut cmd: assert_cmd::Command = cargo_bin_cmd!("rust_bpmn_analyzer_cli").into();
    cmd.env("NO_COLOR", "1");
    cmd
}

#[test]
fn test_help_flag() {
    cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("CLI BPMN Analyzer"))
        .stdout(predicate::str::contains("--file-path"));
}

#[test]
fn test_version_flag() {
    cli()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("rust_bpmn_analyzer_cli"));
}

#[test]
fn test_missing_required_args() {
    cli()
        .assert()
        .failure()
        .stderr(predicate::str::contains("--file-path"));
}

#[test]
fn test_file_not_found() {
    cli()
        .args(["--file-path", "nonexistent.bpmn"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error reading the file"));
}

#[test]
fn test_all_properties_fulfilled() {
    cli()
        .args([
            "--file-path",
            P2_PATH,
            "-p",
            "safeness,option-to-complete,proper-completion,no-dead-activities",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("States:"))
        .stdout(predicate::str::contains("Transitions:"))
        .stdout(predicate::str::contains("fulfilled"))
        .stdout(predicate::str::contains("not fulfilled").not());
}

#[test]
fn test_unfulfilled_properties() {
    cli()
        .args([
            "--file-path",
            P6_STUCK_PATH,
            "-p",
            "safeness,option-to-complete,proper-completion,no-dead-activities",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("not fulfilled"));
}

#[test]
fn test_single_property() {
    cli()
        .args(["--file-path", P2_PATH, "-p", "safeness"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Safeness"))
        .stdout(predicate::str::contains("fulfilled"));
}

#[test]
fn test_no_properties() {
    cli()
        .args(["--file-path", P2_PATH])
        .assert()
        .success()
        .stdout(predicate::str::contains("States:"))
        .stdout(predicate::str::contains("Safeness").not());
}

#[test]
fn test_por_mode() {
    cli()
        .args(["--file-path", P2_PATH, "-p", "safeness", "--por"])
        .assert()
        .success()
        .stdout(predicate::str::contains("States:"))
        .stdout(predicate::str::contains("fulfilled"));
}

#[test]
fn test_por_with_stats() {
    cli()
        .args([
            "--file-path",
            P2_PATH,
            "-p",
            "safeness",
            "--por",
            "--por-stats",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Partial Order Reduction Statistics",
        ))
        .stdout(predicate::str::contains("States processed:"))
        .stdout(predicate::str::contains("Transition reduction:"));
}
